use crate::chunk::*;
use crate::krand::*;
use crate::settings::*;
use crate::kmath::*;
use crate::kimg::*;

/*
lets give this trait thing a try
i wonder if some fancy combinator is possible... sounds slow lol
*/



pub const SEA_LEVEL_F32: f32 = 0.0;
pub const SEA_LEVEL_I32: i32 = 0;

pub trait LevelGenerator: Clone + Send + Sync + 'static {

    fn height(&self, x: f32, z: f32) -> f32;
    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block>;

    fn height_gradient(&self, x: f32, z: f32) -> (f32, Vec2) {
        let h1 = self.height(x,z);
        let hgx = self.height(x + 1.0, z + 0.0);
        let hgz = self.height(x + 0.0, z + 1.0);

        let gradx = h1 - hgx;
        let gradz = h1 - hgz;

        (h1, Vec2{x: gradx, y: gradz})
    }
}


fn fbm1(p: Vec2, seed: u32) -> f32 {
    fgrad2_isotropic(p.x, p.y, seed)
}

fn fbm2(p: Vec2, seed: u32) -> Vec2 {
    Vec2 {
        x: fgrad2_isotropic(p.x, p.y, seed),
        y: fgrad2_isotropic(p.x, p.y, seed + 0x230895F7),
    }
}

fn slow_stop(t: f32) -> f32 {
    1.0 - (1.0 - t)*(1.0 - t)
}


#[derive(Clone)]
pub struct GenNormalCliffy {
    seed: u32,
}

impl GenNormalCliffy {
    pub fn new(seed: u32) -> GenNormalCliffy {
        GenNormalCliffy {seed}
    }


    fn block_underground(&self, x: i32, y: i32, z: i32) -> Block {
        let cavern1_floor_noise = fgrad2_isotropic(0.02 * x as f32, 0.02 * z as f32, self.seed * 23492349);
        let cavern1_ceiling_noise = fgrad2_isotropic(0.02 * x as f32, 0.02 * z as f32, self.seed * 93471753);
        let cavern1_sep = fgrad2_isotropic(0.01 * x as f32, 0.01 * z as f32, self.seed * 93471753);

        let floor = (-120.0 + cavern1_floor_noise * 60.0) as i32;
        let ceiling = (-120.0 + cavern1_ceiling_noise * 60.0) as i32 + (50.0 * cavern1_sep) as i32;

        if y >= ceiling || y < floor {
            Block::Stone
        } else if y == floor {
            Block::BlueFungus
        } else {
            Block::Air
        }
    }

}

impl LevelGenerator for GenNormalCliffy {
    fn height(&self, x: f32, z: f32) -> f32 {
        let lf = 500.0 * grad2_isotropic(0.001 * x, 0.001 * z, 420) - 250.0;

        let height_noise = fgrad2_isotropic(0.005 * x as f32, 0.005 * z as f32, self.seed);
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
        let height = squished + lf + cliff; // good or bad to break da rules, like actual sand squish seems smart
        height// would like to squish beaches but what can ya do
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32 + 0.5, z as f32 + 0.5) as i32;
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;
    
    
                    let block = if y > height {
                        if y >= SEA_LEVEL_I32 {
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
                        self.block_underground(x, y, z)
                    } else {
                        Block::Wat
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}


#[derive(Clone)]
pub struct GenHell {
    seed: u32,
}

impl GenHell {
    pub fn new(seed: u32) -> GenHell {
        GenHell {seed}
    }
}

impl LevelGenerator for GenHell {
    fn height(&self, x: f32, z: f32) -> f32 {    
        let height_noise = fgrad2_isotropic(0.005 * x as f32, 0.005 * z as f32, self.seed) - 0.2;

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

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
    
                
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32 + 0.5, z as f32 + 0.5) as i32;
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
}


#[derive(Clone)]
pub struct GenExp {
    seed: u32,
}

impl GenExp {
    pub fn new(seed: u32) -> GenExp {
        GenExp {
            seed
        }
    }
}


impl LevelGenerator for GenExp {


    fn height(&self, x: f32, z: f32) -> f32 {    
        let height_noise = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed);

        (height_noise - 0.6) * 400.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenErosion {
    seed: u32,
}

impl GenErosion {
    pub fn new(seed: u32) -> GenErosion {
        GenErosion {
            seed
        }
    }
}


impl LevelGenerator for GenErosion {


    fn height(&self, x: f32, z: f32) -> f32 {    

        let h_lf = fgrad2_isotropic_exp(0.0005 * x, 0.0005 * z, self.seed + 71237127);

        // confederate crags, min and max of several so lots of grainy boundaries

        // or fn switches between 4

        // or max function

        // what if you just randomize a bunch of hyperparameters like lacunarity

        let h1 = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + 123124121);
        let h2 = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + 141123121);

        let t_noise = fgrad2_isotropic(0.0025 * x, 0.0025 * z, self.seed + 1341234111);

        let sharpness = grad2_isotropic(0.005 * x, 0.005 * z, self.seed + 2312345) * 0.015;

        let t = saturate(t_noise, 0.5 - sharpness, 0.50 + sharpness);

        /*
        // let t = grad2_isotropic(0.00025 * x, 0.00025 * z, self.seed + 1341234111);

        // maybe cliffs should set a bound
        // maybe erosion should change with water

        // escarpments would be good
        // is erosion even doing anything

        let erosion = fgrad2_isotropic(0.002 * x, 0.002 * z, self.seed).powf(2.0);
        // let erosion = 0.0;
        
        let big_noise = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + 123124121);

        
        let height_noise = big_noise;

        let hard_big = fgrad2_isotropic(0.0003 * x, 0.0003 * z, self.seed * 2342341);
        // let hard_small = fgrad2_isotropic(0.003 * x, 0.003 * z, self.seed * 5134911);

        let hard_small = height_noise.powf(2.0);
        // + correlate with height noise?
        
        let hard = if hard_big > 0.6 || (hard_big > 0.5 && hard_small > 0.6) {
            1.0
        } else {
            0.0
        };

        let h = ((big_noise - 0.5) + height_noise) - (erosion * (1.0 - hard));

        (h - 0.5) * 200.0
        */

        let h_hummocks = lerp(h1, h2, t);
        let h = lerp(h_lf, h_hummocks, 0.5);
        // let h = h1;
        (h - 0.5) * 200.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenErosion2 {
    seed: u32,
}

impl GenErosion2 {
    pub fn new(seed: u32) -> GenErosion2 {
        GenErosion2 {
            seed
        }
    }
}


impl LevelGenerator for GenErosion2 {


    fn height(&self, x: f32, z: f32) -> f32 {    

        let h1 = grad2_isotropic_exp(0.0005 * x, 0.0005 * z, self.seed + 71237127);
        let h2 = 0.5 * grad2_isotropic_exp(0.001 * x, 0.001 * z, self.seed + 61712371);
        let h3_1 = 0.25 * grad2_isotropic_exp(0.002 * x, 0.002 * z, self.seed + 51241123);
        let h3_2 = 0.25 * grad2_isotropic_exp(0.002 * x, 0.002 * z, self.seed + 45347127);
        let h4_1 = 0.125 * grad2_isotropic_exp(0.004 * x, 0.004 * z, self.seed + 63411233);
        let h4_2 = 0.125 * grad2_isotropic_exp(0.004 * x, 0.004 * z, self.seed + 34717247);
        let h5 = 0.0625 * grad2_isotropic_exp(0.008 * x, 0.008 * z, self.seed + 8573419);

        
        let h1 = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + 123124121);
        let h2 = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + 141123121);
        
        let t_noise = fgrad2_isotropic(0.0025 * x, 0.0025 * z, self.seed + 1341234111);
        
        let sharpness = grad2_isotropic(0.005 * x, 0.005 * z, self.seed + 2312345) * 0.015;
        
        let t = saturate(t_noise, 0.5 - sharpness, 0.50 + sharpness);
        let h3 = lerp(h3_1, h3_2, t);
        let h4 = lerp(h4_1, h4_2, t);

        let h = h1 + h2 + h3 + h4;
        (h - 0.5) * 200.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenCrag {
    seed: u32,
}

impl GenCrag {
    pub fn new(seed: u32) -> GenCrag {
        GenCrag {
            seed
        }
    }
}


impl LevelGenerator for GenCrag {


    fn height(&self, x: f32, z: f32) -> f32 {

        let mut max: f32 = 0.0;
        for i in 0..5 {
            let hi = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed + i);
            max = max.max(hi);
        }

        let h = max;

        (h - 0.5) * 200.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}


#[derive(Clone)]
pub struct GenWarp {
    seed: u32,
}

impl GenWarp {
    pub fn new(seed: u32) -> GenWarp {
        GenWarp {
            seed
        }
    }

    fn fbm1(p: Vec2, seed: u32) -> f32 {
        fgrad2_isotropic(p.x, p.y, seed)
    }

    fn fbm2(p: Vec2, seed: u32) -> Vec2 {
        Vec2 {
            x: fgrad2_isotropic(p.x, p.y, seed),
            y: fgrad2_isotropic(p.x, p.y, seed + 0x230895F7),
        }
    }
}


impl LevelGenerator for GenWarp {

    fn height(&self, x: f32, z: f32) -> f32 {
        let p = 0.005 * Vec2 { x, y: z};

        let p_lf = 0.0005 * Vec2{x, y:z};
        let h_lf = GenWarp::fbm1(p_lf + 0.5*GenWarp::fbm2(p, self.seed + 0x12345131), self.seed + 0x42141213);

        let h_mountain = GenWarp::fbm1(p + GenWarp::fbm2(p + GenWarp::fbm2(p + GenWarp::fbm2(p, self.seed + 0x31261343), self.seed + 0x91376513), self.seed + 0x23452337), self.seed);
        let h_mountain_sharp = h_mountain * h_mountain * h_mountain * h_mountain * 5.0;

        let h_mountain = h_mountain.max(h_mountain_sharp); // or softmax

        let p_rough = 0.002 * Vec2 { x, y: z};
        let roughness = GenWarp::fbm1(p_rough, self.seed + 34192313);
        // let roughness = fgrad2_isotropic(0.001 * x, 0.001 * z, self.seed + 34192313);
        let t_mountain = saturate(roughness - 0.1, 0.35, 0.65);

        // let h_plains = 0.5;

        let h_land = h_lf + t_mountain * t_mountain * h_mountain;
        
        // let h_land = lerp(h_lf, h_lf + h_mountain, t_mountain);


        let h_ocean = 0.1;
        let t_ocean = saturate(fgrad2_isotropic(0.0005 * x, 0.0005 * z, self.seed + 34111231), 0.50, 0.7);

        let h = lerp(h_land, h_ocean, t_ocean);

        
        
        // let h = lerp(lf, h_warp, t);
        // let h = lf;

        // let h = GenWarp::fbm1(p, self.seed);

        /*
        let wwarp_coeff = 0.001;
        let wwarp_mag = 0.5;
        let x_wwarp = grad2_isotropic(x * wwarp_coeff, z * wwarp_coeff, self.seed + 523423431) - 0.5;
        let z_wwarp = grad2_isotropic(x * wwarp_coeff, z * wwarp_coeff, self.seed + 735234177) - 0.5;

        // wonder if correlating them is good

        let warp_coeff = 0.001;
        let warp_mag = 10.0;
        let x_warp = grad2_isotropic(x * warp_coeff + x_wwarp * wwarp_mag, z * warp_coeff + z_wwarp * wwarp_mag, self.seed + 123451235) - 0.5;
        let z_warp = grad2_isotropic(x * warp_coeff + x_wwarp * wwarp_mag, z * warp_coeff + z_wwarp * wwarp_mag, self.seed + 541241313) - 0.5;
        let height_noise = fgrad2_isotropic_exp(0.0025 * x + x_warp * warp_mag, 0.0025 * z + z_warp * warp_mag, self.seed);

        (height_noise - 0.5) * 400.0
        */

        (h - 0.4) * 200.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let (height, grad) = self.height_gradient(x as f32, z as f32);
                let height = height as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;

                    // let m = grad.x.abs().max(grad.y.abs());
                    let m = grad.magnitude();
                    let block = match (y - height, y, m) {
                        (dh, y, g) if dh > 0 && y > 0 => Block::Air,
                        (dh, y, g) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y, g) if g > 1.9 => Block::Stone,
                        (dh, y, g) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y, g) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y, g) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[test]
fn test_gen_beach() {

    let beach_t = |x, z| {

        let lf = grad2_isotropic(0.0005 * x, 0.0005 * z, 69);//.powf();


        let beach_start = 0.5;
        let beach_peak = 0.55;

        let beach_t = saturate(lf, beach_start, beach_peak);
        beach_t
    };

    dump_image_fn("beach.png", 
    &beach_t,
    &|c| Vec3::new(c,c,c),
    500, -10000.0, 10000.0,
    500, -10000.0, 10000.0);
}

#[derive(Clone)]
pub struct GenBeach {
    seed: u32,
}

pub struct Beach2d {
    height: f32,
    beachness: f32,
    vegetation: f32,
}

impl GenBeach {
    pub fn new(seed: u32) -> GenBeach {
        GenBeach {
            seed
        }
    }

    fn block_beach(&self, x: i32, y: i32, z: i32, beach_params: &Beach2d) -> Block {
        let h = beach_params.height as i32;
        let dh = y - h;

        let grass_roll = khash_2float(x as u32, z as u32, self.seed);
        let surface_grass = beach_params.vegetation;

        if dh > 0 && y >= 0 {
            Block::Air
        } else if dh > 0 && y < 0 {
            Block::Water
        } else if dh == 0 {
            if surface_grass > 0.1 && grass_roll > 0.5 {
                Block::Grass
            } else {
                Block::Sand
            }
        } else if dh > -10 {
            Block::Sand
        } else {
            Block::Stone
        }
    }

    pub fn vals2d(&self, x: f32, z: f32) -> Beach2d {
        let p = 0.0005 * Vec2::new(x, z);

        let lf = grad2_isotropic(p.x, p.y, self.seed);//.powf();

        let land_noise = fgrad2_isotropic(x * 0.005, z * 0.005, self.seed + 2345823);

        let beach_start = 0.5;
        let beach_peak = 0.55;
        let beach_end = 0.65;

        let ocean_t = saturate(lf, 0.0, beach_start);
        let beach_t = saturate(lf, beach_start, beach_peak);
        let beach_land_t = saturate(lf, beach_peak, beach_end);


        // let mut h = 0.5 * beach_t + 0.5 * beach_t * (60.0 * beach_t).cos();

        // let beach_h = bezier_transect(beach_t, 
        //     // &[   0.1, 0.1, 0.1, 0.15, 0.15, 0.2, 0.2],
        //     &[   1.0, 1.0, 1.0, 1.5, 1.5, 2.0, 2.0],
        //     &[0.0, 0.1, 0.3, 0.1, 0.4,  0.15, 0.5, 0.3], 
        //     &[(Vec2::new(0.3, 0.5), Vec2::new(0.3, 0.5)),
        //     (Vec2::new(0.1, -0.5), Vec2::new(0.1, 0.5)),
        //     (Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5)),
        //     (Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5)),
        //     (Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5)),
        //     (Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5)),
        //     (Vec2::new(0.5, -0.5), Vec2::new(0.5, 0.5)),
        //     ]);

        let beach_h = bezier_transect(beach_t, 
            &[   5.0, 1.0, 2.0, 1.0, 3.0, 4.0],
            &[0.0, 0.1, 0.1, 0.3, 0.25, 0.4, 0.25], 
            &[(Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            (Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0)),
            ]);

        // let beach_h = bezier_transect(beach_t, 
        //     &[   1.0, 1.0, 1.0, 1.0],
        //     &[0.0, 0.5, 0.0, 0.5, 0.0], 
        //     &[(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)),
        //     (Vec2::new(1.0, 1.0), Vec2::new(0.0, 0.0)),
        //     (Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)),
        //     (Vec2::new(1.0, 1.0), Vec2::new(0.0, 0.0)),
        //     ]);

        // let beach_h = bezier_transect(beach_t, 
        //     &[   1.0, 1.0, 1.0, 1.0],
        //     &[0.0, 0.5, 0.0, 0.5, 0.0], 
        //     &[(Vec2::new(0.5, 0.0), Vec2::new(0.5, 1.0)),
        //     (Vec2::new(0.5, 1.0), Vec2::new(0.5, 0.0)),
        //     (Vec2::new(0.5, 0.0), Vec2::new(0.5, 1.0)),
        //     (Vec2::new(0.5, 1.0), Vec2::new(0.5, 0.0)),
        //     ]);

        let h = if beach_t > 0.0 && beach_t < 1.0 {
            beach_h
            // beach_t
        } else if ocean_t < 1.0 {
            -1.0
        } else if beach_t >= 1.0 {            
            lerp(beach_h, land_noise, beach_land_t)
        } else {
            -1.0
            // land_noise.powf(1.5) * 100.0
        };

        // let v = 

        Beach2d {
            height: h * 60.0,
            beachness: beach_t,
            vegetation: beach_t * beach_h,
        }

    }

}


impl LevelGenerator for GenBeach {

    fn height(&self, x: f32, z: f32) -> f32 {
        self.vals2d(x, z).height
        
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let vals2d = self.vals2d(x as f32, z as f32);
                let height = vals2d.height as i32;
                let b = vals2d.beachness;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;

                    let block = match (y - height, y, b) {
                        (dh, y, b) if dh > 0 && y > 0 => Block::Air,
                        (dh, y, b) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y, b) if b < 0.99 => self.block_beach(x, y, z, &vals2d),
                        (dh, y, b) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y, b) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}


#[derive(Clone)]
pub struct GenClassify {
    // porbolem is its a linear classifier, boring!
    seed: u32,
}

impl GenClassify {
    pub fn new(seed: u32) -> GenClassify {
        GenClassify {
            seed
        }
    }
}


impl LevelGenerator for GenClassify {

    fn height(&self, x: f32, z: f32) -> f32 {

        // let d = 0.4;
        let d = fgrad2_isotropic(x, z, self.seed + 23434713);
        let neighs = [(-d, 0.0), (0.0, -d), (d, 0.0), (0.0, d), (0.0, 0.0)];
        let c = neighs.iter()
            // .map(|(nx, ny)| fgrad2_isotropic(x, z, self.seed))
            .map(|(nx, ny)| fgrad2_isotropic(0.005 * x + *nx, 0.005 * z + *ny, self.seed))
            .filter(|x| *x > 0.7)
            .count();

            let h = match c {
            0 => -10.0,
            1 => 2.0,
            2 => 10.0,
            3 => 20.0,
            4 => 30.0,
            5 => 30.0,
            _ => panic!("unreachable")
        };

        

        h
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}
#[derive(Clone)]
pub struct GenTable {
    seed: u32,
}

impl GenTable {
    pub fn new(seed: u32) -> GenTable {
        GenTable {
            seed
        }
    }
}


impl LevelGenerator for GenTable {

    fn height(&self, x: f32, z: f32) -> f32 {
        let tn = fgrad2_isotropic(x * 0.001, z * 0.001, self.seed);

        let tn_adjusted = remap(tn, 0.49, 0.51, 0.0, 1.0).clamp(0.0, 1.0);

        tn_adjusted * 40.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenBlue {
    seed: u32,
}

impl GenBlue {
    pub fn new(seed: u32) -> GenBlue {
        GenBlue {
            seed
        }
    }
}

impl LevelGenerator for GenBlue {
    fn height(&self, x: f32, z: f32) -> f32 {
        // let nxy = |x: f32, z: f32| -> f32 {grad2_isotropic(x * 0.1, z * 0.1, self.seed)};
        //let h = grad2_isotropic(x * 0.1, z * 0.1, self.seed);
        // let h = nxy(x,z) - nxy(x-1.0, z) - nxy(x+1.0,z) - nxy(x, z-1.0) - nxy(x, z+1.0) + 1.0;
        let period = 0.1;
        let h = (0.1 * x).sin().abs() + (0.1 * z).sin().abs();
        10.0 * h
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        // (dh, y) if dh == 0 && y > 120 => Block::Snow,
                        // (dh, y) if dh > -4 && y > 100 => Block::Stone,
                        // (dh, y) if dh == 0 && y > 80 => Block::Dirt,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenIsland {
    seed: u32,
}

impl GenIsland {
    pub fn new(seed: u32) -> GenIsland {
        GenIsland {
            seed
        }
    }
}

impl LevelGenerator for GenIsland {

    // tension between maintaining interesting coastline shapes and having tall mountains in the middle

    fn height(&self, x: f32, z: f32) -> f32 {   
        let island_height_noise = fgrad2_isotropic_exp(0.005 * x, 0.005 * z, self.seed + 234235);
        let r = 500.0;
        let xp = x/r;
        let zp = z/r;
        let mut island_height = 1.0 - (xp*xp + zp*zp).sqrt();

        island_height = if island_height < 0.0 {
            island_height
        } else {
            island_height.powf(3.0)
        };

        
        // ok this was an interesting remapping, not really the effect I was hoping for
        // theres some real big hills too, like maybe you want to modify the height noise thing or do a softmin/softmax by another function
        
        // some smooth/flat areas would be good
        
        // let height_noise = fgrad2_isotropic_exp(0.0025 * x, 0.0025 * z, self.seed);

        // how to get that nice eroded look that real terrains got
        
        let final_height = 100.0 * island_height_noise + 100.0 * island_height;
        
        let on_island = final_height > 0.0;

        let cliff_height = if on_island {
            100.0 * island_height_noise
        } else {
            final_height
        };

        let ct_noise = fgrad2_isotropic(0.001 * x, 0.001 * z, self.seed + 88901917);
        let ct = if !on_island || ct_noise < 0.5 {
            0.0
        } else {
            (ct_noise - 0.5) * 2.0
        };

        lerp(final_height, cliff_height, ct)


        // (height_noise - 0.5) * 400.0
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let height = self.height(x as f32, z as f32) as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;


                    let block = match (y - height, y) {
                        (dh, y) if dh > 0 && y > 0 => Block::Air,
                        (dh, y) if dh > 0 && y <= 0 => Block::Water,
                        // (dh, y) if dh == 0 && y > 120 => Block::Snow,
                        // (dh, y) if dh > -4 && y > 100 => Block::Stone,
                        // (dh, y) if dh == 0 && y > 80 => Block::Dirt,
                        (dh, y) if dh == 0 && y > 4 => Block::Grass,
                        (dh, y) if dh == 0 && y > -4 => Block::Sand,
                        (dh, y) if dh > -4 => Block::Dirt,
                        _ => Block::Stone,
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}

#[derive(Clone)]
pub struct GenMagicMoon {
    seed: u32,
}

struct Moon2DProperties {
    height: f32,
    in_crater: bool,
    grassy: bool,
    lake: bool,
}

impl GenMagicMoon {
    pub fn new(seed: u32) -> GenMagicMoon {
        GenMagicMoon {seed}
    }

    fn props_2d(&self, x: f32, z: f32) -> Moon2DProperties {    
        let height_noise = fgrad2_isotropic(0.005 * x as f32, 0.005 * z as f32, self.seed);
        
        let floor =  0.0;
        let crater_noise = grad2_isotropic(0.005 * x as f32, 0.005 * z as f32, self.seed * 634536);
        let lake_noise = grad2_isotropic(0.05 * x as f32, 0.05 * z as f32, self.seed * 123323);
        let crater_threshold = 0.6;
        let in_crater = crater_noise > crater_threshold;
        let cnn = if in_crater {
            let cnn = (1.0 - crater_noise) / (1.0 - crater_threshold);
            cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn
        } else {
            let cnn = crater_noise / crater_threshold;
            cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn*cnn
        };
        
        let height = if in_crater {
            (floor + 30.0 * height_noise + 16.0) * cnn
        } else {
            floor + 30.0 * height_noise + 16.0 * cnn
        };
        let grassy = in_crater && cnn < 0.25;
        let lake = in_crater && cnn < 0.2 && lake_noise > 0.6;

        Moon2DProperties {
            height,
            in_crater,
            grassy,
            lake,
        }
    }
}



impl LevelGenerator for GenMagicMoon {


    fn height(&self, x: f32, z: f32) -> f32 {    
        self.props_2d(x, z).height
    }

    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let props = self.props_2d(x as f32 + 0.5, z as f32 + 0.5);
                let height = props.height as i32;
            
                for j in 0..S {
                    let idx = k*S + j*S*S + i;
                    let y = oy*S as i32 + j as i32;
    
                    let block = if props.grassy {
                        if props.lake {
                            if y > height {
                                Block::Air
                            } else {
                                Block::Water
                            }
                        } else {
                            if y > height + 1 {
                                Block::Air
                            } else {
                                Block::Grass
                            }
                        }
                    } else {
                        if y > height {
                            Block::Air
                        } else {
                            Block::MoonRock
                        }
                    };
    
                    blocks[idx] = block;
                }
            }
        }
        blocks
    }
}