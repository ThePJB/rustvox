use glow::*;
use crate::krand::*;
use crate::kmath::*;
use bytemuck::*;
use crate::chunk_manager::*;


pub const S: usize = 16;
pub const S_F32: f32 = 16.0;
pub const HALF_S_F32: f32 = 8.0;


pub struct Chunk {
    blocks: Vec<Block>,

    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    num_triangles: i32,
    cc: ChunkCoordinates,
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
    Wat,
}

pub const SEA_LEVEL_F32: f32 = 0.0;
pub const SEA_LEVEL_I32: i32 = 0;

pub fn height(x: f32, z: f32, debug: bool) -> f32 {
    let lf = 500.0 * grad2_isotropic(0.001 * x, 0.001 * z, 420) - 250.0;

    let height_noise = fgrad2_isotropic(0.005 * x as f32, 0.005 * z as f32, 69);
    let initial = 500.0 * height_noise - 250.0;

    let squish_begin = SEA_LEVEL_F32 - 5.0;
    let squish_end =  SEA_LEVEL_F32 + 50.0;

    let squish_factor = 0.25;

    // these are good parameters to throw in
    // have a base height thats quite unaffected etc

    let cliff_pure = grad2_isotropic(0.001 * x, 0.001 * z, 123454321);

    let cliff_score = cliff_pure * height_noise * height_noise;

    let cliff = if cliff_score > 0.2 {
        60.0*cliff_pure*cliff_pure
    } else if cliff_score > 0.1 {
        60.0*cliff_pure*cliff_pure * (cliff_score - 0.1) * 1.0
    } else {
        0.0
    };

    let squished = if initial < squish_begin{
        initial
    } else if initial < squish_end {
        squish_begin + (initial - squish_begin) * squish_factor
    } else {
        squish_begin + (squish_end - squish_begin) * squish_factor + (initial - squish_end)
    };
    if debug {
        println!("x: {} z: {} lf: {} initial: {} squished: {}", x, z, lf, initial, squished);
    }
    squished + lf + cliff // good or bad to break da rules, like actual sand squish seems smart
}

fn generate_blocks_noise(ox: i32, oy: i32, oz: i32) -> Vec<Block> {
    let mut blocks = vec![Block::Air; S*S*S];
    for k in 0..S {
        let z = oz*S as i32 + k as i32;

        for i in 0..S {
            let x = ox*S as i32 + i as i32;
            let height = height(x as f32 + 0.5, z as f32 + 0.5, false) as i32;
            for j in 0..S {
                let idx = k*S + j*S*S + i;
                let y = oy*S as i32 + j as i32;


                let block = if y > height {
                    if y > SEA_LEVEL_I32 {
                        Block::Air
                    } else {
                        Block::Water
                    }
                } else if y < SEA_LEVEL_I32 + 5 && y > height - 3 {
                    Block::Sand
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

pub fn height_hell(x: f32, z: f32, debug: bool) -> f32 {
    let height_noise = fgrad2_isotropic(0.005 * x as f32, 0.005 * z as f32, 69) - 0.2;

    let deep_hole_noise1 = fgrad2_isotropic(0.01 * x, 0.01 * z, 123);
    let deep_hole_noise2 = grad2_isotropic(0.01 * x, 0.01 * z, 321);
    let shallow_hole_noise = grad2_isotropic(0.01 * x, 0.01 * z, 123321);

    let deep_hole = deep_hole_noise1 > 0.6 || deep_hole_noise2 > 0.6;
    let shallow_hole = shallow_hole_noise > 0.5;

    100.0 * height_noise +
    if deep_hole {
        -100.0
    } else {
        0.0
    } +
    if shallow_hole {
        -20.0
    } else {
        0.0
    }
}

fn generate_blocks_hell(ox: i32, oy: i32, oz: i32) -> Vec<Block> {
    let mut blocks = vec![Block::Air; S*S*S];
    for k in 0..S {
        let z = oz*S as i32 + k as i32;

        for i in 0..S {

            
            let x = ox*S as i32 + i as i32;
            let height = height_hell(x as f32 + 0.5, z as f32 + 0.5, false) as i32;
            let deep_hole_noise1 = fgrad2_isotropic(0.01 * (x as f32 + 0.5), 0.01 * (z as f32 + 0.5), 123);
            let deep_hole_noise2 = grad2_isotropic(0.01 * (x as f32 + 0.5), 0.01 * (z as f32 + 0.5), 321);
            let shallow_hole_noise = grad2_isotropic(0.01 * (x as f32 + 0.5), 0.01 * (z as f32 + 0.5), 123321);

            let grass = fgrad2_isotropic(0.02 * (x as f32 + 0.5), 0.02 * (z as f32 + 0.5), 76767654) > 0.5;
        
            let nearly_deep_hole = deep_hole_noise1 > 0.58 || deep_hole_noise2 > 0.58;
            let shallow_hole = shallow_hole_noise > 0.5;

            let do_grass = !nearly_deep_hole && !shallow_hole && grass;
        
            for j in 0..S {
                let idx = k*S + j*S*S + i;
                let y = oy*S as i32 + j as i32;


                let block = if y > height {
                    if y > SEA_LEVEL_I32 {
                        Block::Air
                    } else {
                        Block::Lava
                    }
                } else if y < SEA_LEVEL_I32 + 5 && y > height - 3 {
                    Block::Sand
                } else if y == height {
                    if do_grass {
                        Block::DeadGrass
                    } else {
                        Block::Air
                    }
                } else if y > height - 3 {
                    Block::Dirt
                } else if y <= height - 3 {
                    Block::Hellstone
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






// todo impl drop

impl Chunk {
    pub fn new(gl: &glow::Context, cc: ChunkCoordinates) -> Chunk {
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
            blocks: generate_blocks_hell(cc.x, cc.y, cc.z),
            vao,
            vbo,
            ebo, 
            cc,

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

                            let x = cube_x + i as f32 + S as f32 * self.cc.x as f32;
                            let y = cube_y + j as f32 + S as f32 * self.cc.y as f32;
                            let z = cube_z + k as f32 + S as f32 * self.cc.z as f32;

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