use std::collections::HashMap;

use glow::*;
use crate::chunk::*;
use crate::kmath::*;

/*
Responsibilities:
1. Hold chunks
2. Decide to allocate chunks
3. Decide to deallocate chunks
4. Sort them for rendering?

*/

// idea to handle floating point precision: just mod everything and tell the chunk where it is when we ask it to mesh / draw, but theres an edge case as there always is :)

#[derive(Debug)]
pub struct ChunkCoordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl ChunkCoordinates {
    pub fn containing_world_pos(pos: Vec3) -> ChunkCoordinates {
        let ccf = pos / S as f32;
        let x = ccf.x.floor() as i32;
        let y = ccf.y.floor() as i32;
        let z = ccf.z.floor() as i32;
        ChunkCoordinates {x, y, z}
    }
}

pub struct ChunkManager {
    chunk_map: HashMap<(i32,i32,i32), Chunk>,
}

impl ChunkManager {
    pub fn new(gl: &glow::Context) -> ChunkManager {
        let mut chunk_map = HashMap::new();

        ChunkManager {
            chunk_map,
        }
    }

    pub fn draw(&self, gl: &glow::Context) {
        // todo, sort or whatever

        for (_, chunk) in self.chunk_map.iter() {
            chunk.draw(gl);
        }
    }

    pub fn treadmill(&mut self, gl: &glow::Context, pos: Vec3) {
        let chunk_radius = 6;
        let in_chunk = ChunkCoordinates::containing_world_pos(pos);

        self.chunk_map.retain(|(x,y,z), chunk| {
            let keep =(x - in_chunk.x).abs() <= chunk_radius &&
            (y - in_chunk.y).abs() <= chunk_radius &&
            (z - in_chunk.z).abs() <= chunk_radius;

            if !keep {
                println!("in {:?} unloading {},{},{}", in_chunk, x, y, z);
                chunk.destroy(gl);
            }

            keep
        });

        for i in -chunk_radius..=chunk_radius {
            for j in -chunk_radius..=chunk_radius {
                for k in -chunk_radius..=chunk_radius {
                    let x = in_chunk.x + i;
                    let y = in_chunk.y + j;
                    let z = in_chunk.z + k;
                    if !self.chunk_map.contains_key(&(x,y,z)) {
                        let mut new_chunk = Chunk::new(gl, x, y, z);
                        new_chunk.generate_mesh(gl);
                        self.chunk_map.insert((x,y,z), new_chunk);
                    }
                }
            }
        }
    }

}