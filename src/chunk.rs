use glow::*;
use crate::krand::*;
use bytemuck::*;

pub const S: usize = 16;

fn generate_blocks_noise(ox: i32, oy: i32, oz: i32) -> Vec<Block> {
    let mut blocks = vec![Block::Air; S*S*S];
    for k in 0..S {
        let z = oz*S as i32 + k as i32;

        for i in 0..S {
            let x = ox*S as i32 + i as i32;
            let height = (50.0 * fgrad2_isotropic(0.01 * x as f32, 0.01 * z as f32, 69)) as i32;
            for j in 0..S {
                let idx = k*S + j*S*S + i;
                let y = oy*S as i32 + j as i32;

                let block = if y > height {
                    Block::Air
                } else if y == height {
                    Block::Grass
                } else if y > height - 3 {
                    Block::Dirt
                } else if y <= height - 3 {
                    Block::Stone
                } else {
                    Block::Wat
                };

                blocks[idx] = block;
            }
        }
    }
    blocks
}

fn generate_blocks(ox: i32, oy: i32, oz: i32) -> Vec<Block> {
    let mut blocks = vec![Block::Air; S*S*S];
    for j in 0..S {
        for k in 0..S {
            for i in 0..S {
                let idx = k*S + j*S*S + i;
                let x = ox + i as i32;
                let y = oy + j as i32;
                let z = oz + k as i32;
                blocks[idx] = 
                    if y > 8 {
                        Block::Air
                    } else if y > 7 {
                        Block::Grass
                    } else if y > 4 {
                        Block::Dirt
                    } else {
                        Block::Stone
                    };
            }
        }
    }
    blocks
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Block {
    Air,
    Dirt,
    Grass,
    Stone,
    Wat,
}


pub struct Chunk {
    blocks: Vec<Block>,

    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    num_triangles: i32,

    x: i32,
    y: i32,
    z: i32,
}

// todo impl drop

impl Chunk {
    pub fn new(gl: &glow::Context, x: i32, y: i32, z: i32) -> Chunk {
        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 4*2*3, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 4*2*3, 4*3);
            gl.enable_vertex_attrib_array(1);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            
            (vao, vbo, ebo)
        };

        Chunk {
            blocks: generate_blocks_noise(x, y, z),
            vao,
            vbo,
            ebo, 
            x,
            y,
            z,

            num_triangles: 0,
        }
    }

    pub fn generate_mesh(&mut self, gl: &glow::Context) {
        let cube_verts = [
            0.0f32, 1.0, 1.0,
            0.0, 0.0, 1.0,
            1.0, 0.0, 1.0,
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

            0.0, 1.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 1.0, 1.0,
        ];
        
        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;

        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;

                    let block = self.blocks[idx];
                    if block == Block::Air {
                        continue;
                    }

                    for face in 0..6 {
                        // cull
                        // 0: +z
                        // 1: -z
                        // 2: -y
                        // 3: +y
                        // 4: +x
                        // 5: -x
                        if face == 0 && k < S-1 && self.blocks[idx + S] != Block::Air {
                            continue;
                        }
                        if face == 1 && k > 0 && self.blocks[idx - S] != Block::Air {
                            continue;
                        }
                        if face == 2 && j > 0 && self.blocks[idx - S*S] != Block::Air {
                            continue;
                        }
                        if face == 3 && j < S-1 && self.blocks[idx + S*S] != Block::Air {
                            continue;
                        }
                        if face == 4 && i < S-1 && self.blocks[idx + 1] != Block::Air {
                            continue;
                        }
                        if face == 5 && i > 0 && self.blocks[idx - 1] != Block::Air {
                            continue;
                        }

                        for vert in 0..4 {
                            vertex_buffer.push(cube_verts[face*12 + vert*3] + i as f32 + S as f32 * self.x as f32);
                            vertex_buffer.push(cube_verts[face*12 + vert*3+1] + j as f32 + S as f32 * self.y as f32);
                            vertex_buffer.push(cube_verts[face*12 + vert*3+2] + k as f32 + S as f32 * self.z as f32);
                            
                            let colour = match block {
                                Block::Air => {panic!("unreachable")},
                                Block::Dirt => {[0.7, 0.5, 0.2]},
                                Block::Grass => {[0.0, 1.0, 0.0]},
                                Block::Stone => {[0.6, 0.6, 0.6]},
                                Block::Wat => {[1.0, 0.0, 1.0]},
                
                            };
                            
                            vertex_buffer.push(colour[0]);
                            vertex_buffer.push(colour[1]);
                            vertex_buffer.push(colour[2]);
                            
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