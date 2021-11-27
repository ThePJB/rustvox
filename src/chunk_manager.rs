use std::collections::HashMap;

use glow::*;
use crate::chunk::*;
use crate::kmath::*;
use crate::priority_queue::*;

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
}

pub struct ChunkManager {
    chunk_map: HashMap<ChunkCoordinates, Chunk>,
    chunks_to_generate: PriorityQueue<f32, ChunkCoordinates>,
}

impl ChunkManager {
    pub fn new(gl: &glow::Context) -> ChunkManager {
        let mut chunk_map = HashMap::new();

        ChunkManager {
            chunk_map,
            chunks_to_generate: PriorityQueue::new(),
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        // todo, sort or whatever

        for (_, chunk) in self.chunk_map.iter() {
            chunk.draw(gl);
        }
    }

    pub fn treadmill(&mut self, gl: &glow::Context, pos: Vec3) {
        let chunk_radius = 30;
        let in_chunk = ChunkCoordinates::containing_world_pos(pos);

        self.chunk_map.retain(|cc, chunk| {
            let x = cc.x;
            let y = cc.y;
            let z = cc.z;

            let keep =(x - in_chunk.x).abs() <= chunk_radius &&
            (y - in_chunk.y).abs() <= chunk_radius &&
            (z - in_chunk.z).abs() <= chunk_radius;

            if !keep {
                self.chunks_to_generate.remove(*cc);
                chunk.destroy(gl);
            }

            keep
        });

        for i in -chunk_radius..=chunk_radius {
            for j in -chunk_radius/3..=chunk_radius/3 {
                for k in -chunk_radius..=chunk_radius {
                    let x = in_chunk.x + i;
                    let y = in_chunk.y + j;
                    let z = in_chunk.z + k;

                    let cc = ChunkCoordinates {x,y,z};

                    if !self.chunk_map.contains_key(&cc) {
                        let priority = {
                            let center = cc.center();
                            let height = height_hell(center.x, center.z, false).max(SEA_LEVEL_F32);
                            let distance = (pos - cc.center()).magnitude();
                            if (height - center.y).abs() < 31.0 {
                                distance / 10.0
                            } else {
                                distance
                            }
                        };
                        self.chunks_to_generate.set(priority, cc);
                    }
                }
            }
        }
    }

    pub fn generate_chunks(&mut self, max: i32, gl: &glow::Context) {
        for i in 0..max {
            if let Some(job) = self.chunks_to_generate.remove_min() {
                let mut new_chunk = Chunk::new(gl, job);
                new_chunk.generate_mesh(gl);
                self.chunk_map.insert(job, new_chunk);
            } else {
                return;
            }
        }
    }

}