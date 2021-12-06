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
    pub opaque_mesh: Option<ChunkOpaqueMesh>,
    pub transparent_mesh: Option<ChunkTransparentMesh>,
}

pub struct ChunkOpaqueMesh {
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    pub num_triangles: i32,
}

impl ChunkOpaqueMesh {
    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
            gl.delete_vertex_array(self.vao);
        }
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
}

pub struct ChunkTransparentMesh {
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,

    pub num_triangles: i32,
}

impl ChunkTransparentMesh {
    pub fn destroy(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_buffer(self.vbo);
            gl.delete_buffer(self.ebo);
            gl.delete_vertex_array(self.vao);
        }
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
    Moss,
    BlueFungus,
    YellowFungus,
    Wat,
}

impl Block {
    pub fn is_opaque(&self) -> bool {
        match self {
            Block::Air => false,
            Block::Water => false,
            _ => true,
        }
    }
}

#[derive(Clone)]
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

        let (last_block, last_amount) = self.elements[len-1];
        if last_block == block {
            self.elements[len-1] = (last_block, last_amount + 1);
        } else {
            self.elements.push((block, 1));
        }
    }
}

const CUBE_VERTS: [f32; 72] = [
    1.0, 0.0, 1.0,
    0.0, 0.0, 1.0,
    0.0f32, 1.0, 1.0,
    1.0, 1.0, 1.0,
    
    0.0, 1.0, 0.0,
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 1.0, 0.0,

    0.0, 0.0, 1.0,
    1.0, 0.0, 1.0,
    1.0, 0.0, 0.0,
    0.0, 0.0, 0.0,

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

impl ChunkData {
    pub fn new(cc: ChunkCoordinates, level_gen: &impl LevelGenerator) -> ChunkData {
        ChunkData {
            blocks: level_gen.generate_blocks(cc.x, cc.y, cc.z),
            cc,
        }
    }    

    pub fn faces_rle(&self) -> Vec<BlockRLE> {
        let mut ret = vec![BlockRLE::new(); 6*S*S];

        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;
                    let block = self.blocks[idx];


                    for face in 0..6 {
                        let ret_idx = face * (i + S*k);

                        if !block.is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if block == Block::Water {
                            panic!("shouldnt be water retard")
                        }

                        // cull
                        // 0: +z
                        // 1: -z
                        // 2: -y
                        // 3: +y
                        // 4: +x
                        // 5: -x
                        if face == 0 && k < S-1 && self.blocks[idx + S].is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if face == 1 && k > 0 && self.blocks[idx - S].is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if face == 2 && j > 0 && self.blocks[idx - S*S].is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if face == 3 && j < S-1 && self.blocks[idx + S*S].is_opaque(){
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if face == 4 && i < S-1 && self.blocks[idx + 1].is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }
                        if face == 5 && i > 0 && self.blocks[idx - 1].is_opaque() {
                            ret[ret_idx].push(Block::Air);
                            continue;
                        }

                        ret[ret_idx].push(block);
                    }
                }
            }
        }
        ret
    }

    pub fn new_opaque_mesh_rle(&self, gl: &glow::Context) -> Option<ChunkOpaqueMesh> {
        let face_RLEs = self.faces_rle();

        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;

        for face in 0..6 {
            for i in 0..S {
                for k in 0..S {
                    let mut j = 0;
                    for (block_type, quantity) in face_RLEs[face * (i + S*k)].elements.iter() {

                        if *block_type == Block::Air {
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
                            let cube_x = CUBE_VERTS[face*12 + vert*3];
                            let cube_y = CUBE_VERTS[face*12 + vert*3+1];
                            let cube_z = CUBE_VERTS[face*12 + vert*3+2];

                            let x = cube_x + i as f32 + S as f32 * self.cc.x as f32;
                            let y = cube_y* *quantity as f32 + j as f32 + S as f32 * self.cc.y as f32;
                            let z = cube_z + k as f32 + S as f32 * self.cc.z as f32;

                            vertex_buffer.push(x);
                            vertex_buffer.push(y);
                            vertex_buffer.push(z);
                            
                            let colour = match block_type {
                                Block::Air |
                                Block::Water => {panic!("unreachable")},
                                Block::Dirt => {[0.7, 0.5, 0.2, 1.0]},
                                Block::Grass => {[0.0, 1.0, 0.0, 1.0]},
                                Block::Stone => {[0.6, 0.6, 0.6, 1.0]},
                                Block::Sand => {[1.0, 1.0, 0.0, 1.0]},
                                Block::Lava => {[1.0, 0.0, 0.0, 1.0]},
                                Block::Hellstone => {[0.6, 0.2, 0.2, 1.0]},
                                Block::DeadGrass => {[0.5, 0.7, 0.0, 1.0]},
                                Block::MoonRock => {[0.9, 0.9, 0.7, 1.0]},
                                Block::Moss => {[0.0, 0.7, 0.0, 1.0]},
                                Block::BlueFungus => {[0.0, 0.7, 1.0, 1.0]},
                                Block::YellowFungus => {[0.5, 1.0, 0.1, 1.0]},
                                Block::Wat => {[1.0, 0.0, 1.0, 1.0]},
                
                            };
                            
                            vertex_buffer.push(colour[0]);
                            vertex_buffer.push(colour[1]);
                            vertex_buffer.push(colour[2]);
                            vertex_buffer.push(colour[3]);

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

                        j += quantity;
                    }
                }
            }
        }


        let num_triangles = element_buffer.len() as i32;
        if num_triangles == 0 {
            return None;
        }

        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            let float_size = 4;
            let total_floats = 3 + 4 + 3;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, float_size*total_floats, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, float_size*total_floats, float_size*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, float_size*total_floats, float_size*7);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            
            (vao, vbo, ebo)
        };
        
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertex_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&element_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }        

        Some(ChunkOpaqueMesh {
            num_triangles,
            vao,
            vbo,
            ebo,
        })
    }
    
    fn opaque_buffers_opt(&self) -> (Vec<f32>, Vec<u32>) {
        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;

        let mut push_quad = |verts: [Vec3; 4], normal: Vec3, block: Block, index: u32| {
            let colour = match block {
                Block::Air |
                Block::Water => {panic!("unreachable")},
                Block::Dirt => {[0.7, 0.5, 0.2, 1.0]},
                Block::Grass => {[0.0, 1.0, 0.0, 1.0]},
                Block::Stone => {[0.6, 0.6, 0.6, 1.0]},
                Block::Sand => {[1.0, 1.0, 0.0, 1.0]},
                Block::Lava => {[1.0, 0.0, 0.0, 1.0]},
                Block::Hellstone => {[0.6, 0.2, 0.2, 1.0]},
                Block::DeadGrass => {[0.5, 0.7, 0.0, 1.0]},
                Block::MoonRock => {[0.9, 0.9, 0.7, 1.0]},
                Block::Moss => {[0.0, 0.7, 0.0, 1.0]},
                Block::BlueFungus => {[0.0, 0.7, 1.0, 1.0]},
                Block::YellowFungus => {[0.5, 1.0, 0.1, 1.0]},
                Block::Wat => {[1.0, 0.0, 1.0, 1.0]},
            };
            
            for idx in 0..4 {
                vertex_buffer.push(verts[idx].x);
                vertex_buffer.push(verts[idx].y);
                vertex_buffer.push(verts[idx].z);
    
                vertex_buffer.push(colour[0]);
                vertex_buffer.push(colour[1]);
                vertex_buffer.push(colour[2]);
                vertex_buffer.push(colour[3]);
    
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
        };

        {  
            // +Z
            let face = 0;
            let normal = Vec3::new(0.0, 0.0, 1.0);
            for j in 0..S {
                for k in 0..S {
                    let mut greed_size = 1;
                    for i in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = i < S-1 && self.blocks[idx + 1] == self.blocks[idx] && !(k < S-1 && self.blocks[idx + S+1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if k < S-1 && self.blocks[idx + S].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }

        {  
            // -Z
            let face = 1;
            let normal = Vec3::new(0.0, 0.0, -1.0);
            for j in 0..S {
                for k in 0..S {
                    let mut greed_size = 1;
                    for i in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = i < S-1 && self.blocks[idx + 1] == self.blocks[idx] && !(k > 0 && self.blocks[idx - S+1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if k > 0 && self.blocks[idx - S].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }
        {  
            // -Y
            let face = 2;
            let normal = Vec3::new(0.0, -1.0, 0.0);
            for j in 0..S {
                for k in 0..S {
                    let mut greed_size = 1;
                    for i in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = i < S-1 && self.blocks[idx + 1] == self.blocks[idx] && !(j > 0 && self.blocks[idx - S*S+1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if j > 0 && self.blocks[idx - S*S].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }
        {  
            // +Y
            let face = 3;
            let normal = Vec3::new(0.0, 1.0, 0.0);
            for j in 0..S {
                for k in 0..S {
                    let mut greed_size = 1;
                    for i in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = i < S-1 && self.blocks[idx + 1] == self.blocks[idx] && !(j < S-1 && self.blocks[idx + S*S+1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if j < S-1 && self.blocks[idx + S*S].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3]*greed_size as f32 + i as f32 - (greed_size-1) as f32 as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2] + k as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }
        {  
            // +X
            let face = 4;
            let normal = Vec3::new(1.0, 0.0, 0.0);
            for j in 0..S {
                for i in 0..S {
                let mut greed_size = 1;
                    for k in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = k < S-1 && self.blocks[idx + S] == self.blocks[idx] && !(i < S-1 && self.blocks[idx + S+1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if i < S-1 && self.blocks[idx + 1].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }
        {  
            // -X
            let face = 5;
            let normal = Vec3::new(-1.0, 0.0, 0.0);
            for j in 0..S {
                for i in 0..S {
                let mut greed_size = 1;
                    for k in 0..S {
                        let idx = k*S + j*S*S + i;

                        if !self.blocks[idx].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let can_greed = k < S-1 && self.blocks[idx + S] == self.blocks[idx] && !(i > 0 && self.blocks[idx + S-1].is_opaque());
                        // let can_greed = false;

                        if can_greed {
                            greed_size += 1;
                            continue;
                        }   // else actually mesh

                        if i > 0 && self.blocks[idx - 1].is_opaque() {
                            greed_size = 1;
                            continue;
                        }

                        let verts = [
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 0*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 0*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 0*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 1*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 1*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 1*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 2*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 2*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 2*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                            Vec3 {
                                x: CUBE_VERTS[face*12 + 3*3] + i as f32 + S as f32 * self.cc.x as f32,
                                y: CUBE_VERTS[face*12 + 3*3+1] + j as f32 + S as f32 * self.cc.y as f32,
                                z: CUBE_VERTS[face*12 + 3*3+2]*greed_size as f32 + k as f32 - (greed_size-1) as f32 + S as f32 * self.cc.z as f32,
                            }, 
                        ];
                        
                        push_quad(verts, normal, self.blocks[idx], index);
                        index += 4;

                        greed_size = 1;
                    }
                }
            }
        }

        (vertex_buffer, element_buffer)
    }

    fn opaque_buffers(&self) -> (Vec<f32>, Vec<u32>) {
        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;


        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;

                    let block = self.blocks[idx];
                    if !block.is_opaque() {
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
                        if face == 0 && k < S-1 && self.blocks[idx + S].is_opaque() {
                            continue;
                        }
                        if face == 1 && k > 0 && self.blocks[idx - S].is_opaque() {
                            continue;
                        }
                        if face == 2 && j > 0 && self.blocks[idx - S*S].is_opaque() {
                            continue;
                        }
                        if face == 3 && j < S-1 && self.blocks[idx + S*S].is_opaque(){
                            continue;
                        }
                        if face == 4 && i < S-1 && self.blocks[idx + 1].is_opaque() {
                            continue;
                        }
                        if face == 5 && i > 0 && self.blocks[idx - 1].is_opaque() {
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
                            let cube_x = CUBE_VERTS[face*12 + vert*3];
                            let cube_y = CUBE_VERTS[face*12 + vert*3+1];
                            let cube_z = CUBE_VERTS[face*12 + vert*3+2];

                            let x = cube_x + i as f32 + S as f32 * self.cc.x as f32;
                            let y = cube_y + j as f32 + S as f32 * self.cc.y as f32;
                            let z = cube_z + k as f32 + S as f32 * self.cc.z as f32;

                            vertex_buffer.push(x);
                            vertex_buffer.push(y);
                            vertex_buffer.push(z);
                            
                            let colour = match block {
                                Block::Air |
                                Block::Water => {panic!("unreachable")},
                                Block::Dirt => {[0.7, 0.5, 0.2, 1.0]},
                                Block::Grass => {[0.0, 1.0, 0.0, 1.0]},
                                Block::Stone => {[0.6, 0.6, 0.6, 1.0]},
                                Block::Sand => {[1.0, 1.0, 0.0, 1.0]},
                                Block::Lava => {[1.0, 0.0, 0.0, 1.0]},
                                Block::Hellstone => {[0.6, 0.2, 0.2, 1.0]},
                                Block::DeadGrass => {[0.5, 0.7, 0.0, 1.0]},
                                Block::MoonRock => {[0.9, 0.9, 0.7, 1.0]},
                                Block::Moss => {[0.0, 0.7, 0.0, 1.0]},
                                Block::BlueFungus => {[0.0, 0.7, 1.0, 1.0]},
                                Block::YellowFungus => {[0.5, 1.0, 0.1, 1.0]},
                                Block::Wat => {[1.0, 0.0, 1.0, 1.0]},
                
                            };
                            
                            vertex_buffer.push(colour[0]);
                            vertex_buffer.push(colour[1]);
                            vertex_buffer.push(colour[2]);
                            vertex_buffer.push(colour[3]);

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

        (vertex_buffer, element_buffer)
    }

    pub fn new_opaque_mesh(&self, gl: &glow::Context) -> Option<ChunkOpaqueMesh> {
        let (vertex_buffer, element_buffer) = self.opaque_buffers_opt();

        let num_triangles = element_buffer.len() as i32;
        if num_triangles == 0 {
            return None;
        }

        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            let float_size = 4;
            let total_floats = 3 + 4 + 3;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, float_size*total_floats, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, float_size*total_floats, float_size*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, float_size*total_floats, float_size*7);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            
            (vao, vbo, ebo)
        };
        
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertex_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&element_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }        

        Some(ChunkOpaqueMesh {
            num_triangles,
            vao,
            vbo,
            ebo,
        })

    }

    pub fn new_transparent_mesh(&self, gl: &glow::Context) -> Option<ChunkTransparentMesh> {
        let mut vertex_buffer = Vec::new();
        let mut element_buffer = Vec::new();

        let mut index: u32 = 0;

        for j in 0..S {
            for k in 0..S {
                for i in 0..S {
                    let idx = k*S + j*S*S + i;

                    let block = self.blocks[idx];
                    if block != Block::Water {
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
                        if face == 0 && k < S-1 && self.blocks[idx + S] == Block::Water {
                            continue;
                        }
                        if face == 1 && k > 0 && self.blocks[idx - S] == Block::Water {
                            continue;
                        }
                        if face == 2 && j > 0 && self.blocks[idx - S*S] == Block::Water {
                            continue;
                        }
                        if face == 3 && j < S-1 && self.blocks[idx + S*S] == Block::Water {
                            continue;
                        }
                        if face == 4 && i < S-1 && self.blocks[idx + 1] == Block::Water {
                            continue;
                        }
                        if face == 5 && i > 0 && self.blocks[idx - 1] == Block::Water {
                            continue;
                        }

                        // water above hard to shore up with chunk boundaries...

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
                            let cube_x = CUBE_VERTS[face*12 + vert*3];
                            let cube_y = CUBE_VERTS[face*12 + vert*3+1];
                            let cube_z = CUBE_VERTS[face*12 + vert*3+2];

                            let x = cube_x + i as f32 + S as f32 * self.cc.x as f32;
                            let y = cube_y + j as f32 + S as f32 * self.cc.y as f32;
                            let z = cube_z + k as f32 + S as f32 * self.cc.z as f32;

                            vertex_buffer.push(x);
                            vertex_buffer.push(y);
                            vertex_buffer.push(z);
                            
                            let colour = match block {
                                Block::Water => {[0.0, 0.0, 1.0, 0.5]},
                                _ => {panic!("unreachable")},
                            };
                            
                            vertex_buffer.push(colour[0]);
                            vertex_buffer.push(colour[1]);
                            vertex_buffer.push(colour[2]);
                            vertex_buffer.push(0.5);

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

        let num_triangles = element_buffer.len() as i32;
        if num_triangles == 0 {
            return None;
        }

        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let ebo = gl.create_buffer().unwrap();
            gl.bind_vertex_array(Some(vao));
            let float_size = 4;
            let total_floats = 3 + 4 + 3;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, float_size*total_floats, 0);
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, float_size*total_floats, float_size*3);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, float_size*total_floats, float_size*7);
            gl.enable_vertex_attrib_array(2);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            
            (vao, vbo, ebo)
        };
        
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertex_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, cast_slice(&element_buffer), glow::STATIC_DRAW);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        }        

        Some(ChunkTransparentMesh {
            num_triangles,
            vao,
            vbo,
            ebo,
        })
    }
}


impl Chunk {
    pub fn new(gl: &glow::Context, cd: ChunkData) -> Chunk {
        let opaque_mesh = cd.new_opaque_mesh(gl);
        let transparent_mesh = cd.new_transparent_mesh(gl);
        Chunk {
            data: cd, 
            opaque_mesh, 
            transparent_mesh,
        }        
    }

    pub fn destroy(&mut self, gl: &glow::Context) {
        if let Some(opaque) = &self.opaque_mesh {
            opaque.destroy(gl);
        }
        if let Some(transparent) = &self.transparent_mesh {
            transparent.destroy(gl);
        }
    }
}