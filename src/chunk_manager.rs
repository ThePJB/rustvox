use std::collections::HashMap;

use glow::*;
use glam::Mat4;
use crate::chunk::*;
use crate::kmath::*;
use crate::priority_queue::*;
use crate::world_gen::*;
use crate::settings::*;
use crossbeam::*;
use crossbeam_channel::*;
use std::collections::HashSet;

/*
Responsibilities:
1. Hold chunks
2. Decide to allocate chunks
3. Decide to deallocate chunks
4. Sort them for rendering?

*/

// idea to handle floating point precision: just mod everything and tell the chunk where it is when we ask it to mesh / draw, but theres an edge case as there always is :)

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ChunkCoordinates {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoordinates {
    pub fn containing_world_pos(pos: Vec3) -> ChunkCoordinates {
        let ccf = pos / S as f32;
        let x = ccf.x.floor() as i32;
        let y = ccf.y.floor() as i32;
        let z = ccf.z.floor() as i32;
        ChunkCoordinates {x, y, z}
    }

    pub fn center(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 * S_F32 + HALF_S_F32,
            self.y as f32 * S_F32 + HALF_S_F32,
            self.z as f32 * S_F32 + HALF_S_F32,
        )
    }

    pub fn corners(&self) -> [Vec3; 8] {
        let x = self.x as f32 * S_F32;
        let y = self.y as f32 * S_F32;
        let z = self.z as f32 * S_F32;
        
        [
            Vec3::new(x,y,z),
            Vec3::new(x,y,z+S_F32),
            Vec3::new(x,y+S_F32,z),
            Vec3::new(x,y+S_F32,z+S_F32),
            Vec3::new(x+S_F32,y,z),
            Vec3::new(x+S_F32,y,z+S_F32),
            Vec3::new(x+S_F32,y+S_F32,z),
            Vec3::new(x+S_F32,y+S_F32,z+S_F32),
        ]
    }
}

pub struct ChunkManager {
    pub chunk_map: HashMap<ChunkCoordinates, Chunk>,
    //chunks_to_generate: PriorityQueue<f32, ChunkCoordinates>,

    job_sender: Sender<ChunkCoordinates>,
    chunk_receiver: Receiver<ChunkData>,    // might be doing unnecessary copying
    loading: HashSet<ChunkCoordinates>,
}

const N_WORKERS: usize = 6;
impl ChunkManager {
    pub fn new(gl: &glow::Context, gen: &impl LevelGenerator) -> ChunkManager {
        let mut chunk_map = HashMap::new();

        let (job_sender, job_receiver) = unbounded();
        let (chunk_sender, chunk_receiver) = unbounded();

        for i in 0..N_WORKERS {
            let job_receiver =  job_receiver.clone();
            let chunk_sender = chunk_sender.clone();
            let gen = gen.clone();
            std::thread::spawn(move || {
                let thread_gen = gen.clone();

                loop {
                    let job = job_receiver.recv().unwrap();
                    let chunk_data = ChunkData::new(job, &thread_gen);
                    chunk_sender.send(chunk_data).unwrap();
                }
            });
        }

        ChunkManager {
            chunk_map,
            job_sender,
            chunk_receiver,
            loading: HashSet::new(),
        }
    }

    pub fn draw(&self, gl: &glow::Context, pos: Vec3, look: Vec3, up: Vec3, right: Vec3, fovx: f32, fovy: f32) {

        println!("pos: {}\nlook: {}\n up: {}\n right: {}\n fovx: {}\n fovy: {}", pos, look, up, right, fovx, fovy);

        let vp_up = look.cross(right);

        let  nbot = -look.rotate_about_vec3(right, -fovy).cross(right).normalize();
        let  ntop = look.rotate_about_vec3(right, fovy).cross(right).normalize();
        let  nleft = -look.rotate_about_vec3(vp_up, -fovx).cross(vp_up).normalize();
        let  nright = look.rotate_about_vec3(vp_up, fovx).cross(vp_up).normalize();

        let test_point = |p: Vec3| {
            nbot.dot(p - pos) > 0.0 &&
            ntop.dot(p - pos) > 0.0 &&
            nleft.dot(p - pos) > 0.0 &&
            nright.dot(p - pos) > 0.0
        };

        let mut draw_list: Vec<&Chunk> = self.chunk_map.iter().filter(|(cc, c)| {
            let corners = cc.corners();
            for corner in corners {
                if test_point(corner) {
                    return true;
                }
            }
            return false;
        }).map(|x| x.1).collect();

        // println!("draw {} / {}", draw_list.len(), self.chunk_map.len());
        
        draw_list.sort_unstable_by(|chunk1, chunk2| {
            let dist1 = (chunk1.data.cc.center() - pos).square_distance();
            let dist2 = (chunk2.data.cc.center() - pos).square_distance();
            dist1.partial_cmp(&dist2).unwrap()
        });

        for chunk in draw_list.iter().rev() {
            if let Some(m) = &chunk.opaque_mesh {
                m.draw(gl);
            }
        }

        for chunk in draw_list.iter() {
            if let Some(m) = &chunk.transparent_mesh {
                m.draw(gl);
            }
        }
    }

    pub fn treadmill(&mut self, gl: &glow::Context, pos: Vec3, gen: &impl LevelGenerator) {
        let in_chunk = ChunkCoordinates::containing_world_pos(pos);

        self.chunk_map.retain(|cc, chunk| {
            let x = cc.x;
            let y = cc.y;
            let z = cc.z;

            let keep =(x - in_chunk.x).abs() <= CHUNK_RADIUS &&
            (y - in_chunk.y).abs() <= CHUNK_RADIUS/3 &&
            (z - in_chunk.z).abs() <= CHUNK_RADIUS;

            if !keep {
                chunk.destroy(gl);
            }

            keep
        });

        // post jobs
        for i in -CHUNK_RADIUS..=CHUNK_RADIUS {
            for j in -CHUNK_RADIUS/3..=CHUNK_RADIUS/3 {
                for k in -CHUNK_RADIUS..=CHUNK_RADIUS {
                    let x = in_chunk.x + i;
                    let y = in_chunk.y + j;
                    let z = in_chunk.z + k;

                    let cc = ChunkCoordinates {x,y,z};

                    if !self.chunk_map.contains_key(&cc) && !self.loading.contains(&cc) {
                        self.job_sender.send(cc);
                        self.loading.insert(cc);

                        /*
                        let priority = {
                            let center = cc.center();
                            let height = gen.height(center.x, center.z).max(SEA_LEVEL_F32);
                            let distance = (pos - cc.center()).magnitude();
                            if (height - center.y).abs() < 31.0 {
                                distance / 10.0
                            } else {
                                distance
                            }
                        };
                        self.chunks_to_generate.set(priority, cc);
                        */
                    }
                }
            }
        }

        let mut chunks_this_frame = 0;
        // reap chunks
        while let Ok(chunk_data) = self.chunk_receiver.try_recv() {
            let new_chunk = Chunk::new(gl, chunk_data);
            // new_chunk.generate_mesh(gl);
            self.loading.remove(&new_chunk.data.cc);
            self.chunk_map.insert(new_chunk.data.cc, new_chunk);
            chunks_this_frame += 1;
            if chunks_this_frame > CHUNKS_PER_FRAME {
                break;
            }
        }
    }

    /*
    OK how are we doing this, is shit getting copied around or just references?
    */

    /*
    pub fn generate_chunks(&mut self, max: i32, gl: &glow::Context, gen: &impl LevelGenerator) {
        for i in 0..max {
            if let Some(job) = self.chunks_to_generate.remove_min() {
                let mut new_chunk_data = ChunkData::new(job, gen);
                let mut new_chunk = Chunk::new(gl, new_chunk_data);
                new_chunk.generate_mesh(gl);
                self.chunk_map.insert(job, new_chunk);
            } else {
                return;
            }
        }
    }
    */

}