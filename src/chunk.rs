use glow::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Block {
    Air,
    Dirt,
    Grass,
    Stone,
}

const S: usize = 16;

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
            
            (vao, vbo, ebo)
        };

        let mut blocks = vec![Block::Air; S*S*S];
        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;
                    blocks[idx] = 
                        if j > 8 {
                            Block::Air
                        } else if j > 7 {
                            Block::Grass
                        } else if j > 4 {
                            Block::Dirt
                        } else {
                            Block::Stone
                        };
                }
            }
        }
        Chunk {
            blocks,
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

        let mut index: u16 = 0;

        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;

                    let block = self.blocks[idx];
                    if block == Block::Air {
                        continue;
                    }

                    for face in 0..6 {
                        for vert in 0..4 {
                            vertex_buffer.push(cube_verts[face*12 + vert*3] + i as f32 + S as f32 * self.x as f32);
                            vertex_buffer.push(cube_verts[face*12 + vert*3+1] + j as f32 + S as f32 * self.y as f32);
                            vertex_buffer.push(cube_verts[face*12 + vert*3+2] + k as f32 + S as f32 * self.z as f32);
                            
                            let colour = match block {
                                Block::Air => {panic!("unreachable")},
                                Block::Dirt => {[0.7, 0.5, 0.2]},
                                Block::Grass => {[0.0, 1.0, 0.0]},
                                Block::Stone => {[0.6, 0.6, 0.6]},
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
            let vertex_u8 = {
                let (ptr, len, cap) = vertex_buffer.into_raw_parts();
                Vec::from_raw_parts(ptr as *mut u8, len*4, cap*4)
            };
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertex_u8, glow::STATIC_DRAW);
            
            let element_u8 = {
                let (ptr, len, cap) = element_buffer.into_raw_parts();
                Vec::from_raw_parts(ptr as *mut u8, len*2, cap*2)
            };
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, &element_u8, glow::STATIC_DRAW);
        }
        

    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.draw_elements(glow::TRIANGLES, self.num_triangles, glow::UNSIGNED_SHORT, 0);
        }
    }
}