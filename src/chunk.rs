use glow::*;
use crate::krand::*;
use crate::kmath::*;
use bytemuck::*;
use crate::chunk_manager::*;
use crate::world_gen::*;
use crate::settings::*;



// no gl stuff here please
pub struct ChunkData {
    pub blocks: Vec<Block>,
    pub cc: ChunkCoordinates,
}

pub struct Chunk {
    pub data: ChunkData,

    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    num_triangles: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Block {
    Air,
    Dirt,
    Grass,
    Stone,
    Water,
    Sand,
    Lava,
    Hellstone,
    DeadGrass,
    MoonRock,
    Wat,
}

pub struct BlockRLE {
    elements: Vec<(Block, u32)>,
}

impl BlockRLE {
    pub fn new() -> BlockRLE {
        BlockRLE {
            elements: Vec::new(),
        }
    }

    pub fn push(&mut self, block: Block) {
        let len = self.elements.len();
        if len == 0 {
            self.elements.push((block, 1));
            return;
        }

        let (last_block, last_amount) = self.elements[len];
        if last_block == block {
            self.elements[len] = (last_block, last_amount + 1);
        } else {
            self.elements.push((block, 1));
        }
    }
}

impl ChunkData {
    pub fn new(cc: ChunkCoordinates, level_gen: &impl LevelGenerator) -> ChunkData {
        ChunkData {
            blocks: level_gen.generate_blocks(cc.x, cc.y, cc.z),
            cc,
        }
    }    
}


impl Chunk {
    pub fn new(gl: &glow::Context, cd: ChunkData) -> Chunk {
        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*3*3, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 4*3*3, 4*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, 4*3*3, 4*6);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            
            (vao, vbo, ebo)
        };

        Chunk {
            data: cd,
            vao,
            vbo,
            ebo, 
            num_triangles: 0,
        }
    }


    pub fn generate_mesh(&mut self, gl: &glow::Context) {
        let cube_verts = [
            1.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0f32, 1.0, 1.0,
            1.0, 1.0, 1.0,
            
            0.0, 1.0, 0.0,
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 1.0, 0.0,

            0.0, 0.0, 1.0,
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 1.0,

            0.0, 1.0, 1.0,
            0.0, 1.0, 0.0,
            1.0, 1.0, 0.0,
            1.0, 1.0, 1.0,

            1.0, 1.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 1.0,
            1.0, 1.0, 1.0,

            0.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 1.0,
            0.0, 0.0, 1.0,
        ];
        
        // maybe make mesh greedy a separate fn
        // my bottleneck is probably chunk generation, that might be a more pressing thing to work on

        /*
        let mut faces_pz = BlockRLE::new();
        let mut faces_mz = BlockRLE::new();
        let mut faces_my = BlockRLE::new();
        let mut faces_py = BlockRLE::new();
        let mut faces_px = BlockRLE::new();
        let mut faces_mx = BlockRLE::new();
        */

        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;

        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;

                    let block = self.data.blocks[idx];
                    if block == Block::Air {
                        continue;
                    }

                    for face in 0..6 {
                        if face == 0 {
                            
                        }

                        // cull
                        // 0: +z
                        // 1: -z
                        // 2: -y
                        // 3: +y
                        // 4: +x
                        // 5: -x
                        if face == 0 && k < S-1 && self.data.blocks[idx + S] != Block::Air {
                            continue;
                        }
                        if face == 1 && k > 0 && self.data.blocks[idx - S] != Block::Air {
                            continue;
                        }
                        if face == 2 && j > 0 && self.data.blocks[idx - S*S] != Block::Air {
                            continue;
                        }
                        if face == 3 && j < S-1 && self.data.blocks[idx + S*S] != Block::Air {
                            continue;
                        }
                        if face == 4 && i < S-1 && self.data.blocks[idx + 1] != Block::Air {
                            continue;
                        }
                        if face == 5 && i > 0 && self.data.blocks[idx - 1] != Block::Air {
                            continue;
                        }

                        let normal = match face {
                            0 => Vec3::new(0.0, 0.0, 1.0),
                            1 => Vec3::new(0.0, 0.0, -1.0),
                            2 => Vec3::new(0.0, -1.0, 0.0),
                            3 => Vec3::new(0.0, 1.0, 0.0),
                            4 => Vec3::new(1.0, 0.0, 0.0),
                            5 => Vec3::new(-1.0, 0.0, 0.0),
                            _ => panic!("unreachable"),
                        };

                        for vert in 0..4 {
                            let cube_x = cube_verts[face*12 + vert*3];
                            let cube_y = cube_verts[face*12 + vert*3+1];
                            let cube_z = cube_verts[face*12 + vert*3+2];

                            let x = cube_x + i as f32 + S as f32 * self.data.cc.x as f32;
                            let y = cube_y + j as f32 + S as f32 * self.data.cc.y as f32;
                            let z = cube_z + k as f32 + S as f32 * self.data.cc.z as f32;

                            vertex_buffer.push(x);
                            vertex_buffer.push(y);
                            vertex_buffer.push(z);
                            
                            let colour = match block {
                                Block::Air => {panic!("unreachable")},
                                Block::Dirt => {[0.7, 0.5, 0.2]},
                                Block::Grass => {[0.0, 1.0, 0.0]},
                                Block::Stone => {[0.6, 0.6, 0.6]},
                                Block::Water => {[0.0, 0.0, 1.0]},
                                Block::Sand => {[1.0, 1.0, 0.0]},
                                Block::Lava => {[1.0, 0.0, 0.0]},
                                Block::Hellstone => {[0.6, 0.2, 0.2]},
                                Block::DeadGrass => {[0.5, 0.7, 0.0]},
                                Block::MoonRock => {[0.9, 0.9, 0.7]},
                                Block::Wat => {[1.0, 0.0, 1.0]},
                
                            };
                            
                            vertex_buffer.push(colour[0]);
                            vertex_buffer.push(colour[1]);
                            vertex_buffer.push(colour[2]);

                            vertex_buffer.push(normal.x);
                            vertex_buffer.push(normal.y);
                            vertex_buffer.push(normal.z);
                        }
                        element_buffer.push(index);
                        element_buffer.push(index+1);
                        element_buffer.push(index+2);
                        element_buffer.push(index);
                        element_buffer.push(index+2);
                        element_buffer.push(index+3);
                        index += 4;
                    }
                }
            }
        }

        self.num_triangles = element_buffer.len() as i32;
        
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertex_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&element_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }

        //drop(vertex_buffer);
        //drop(element_buffer);
    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.draw_elements(glow::TRIANGLES, self.num_triangles, glow::UNSIGNED_INT, 0);
            
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);

        }
    }

    pub fn destroy(&mut self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
            gl.delete_vertex_array(self.vao);
        }
    }
}