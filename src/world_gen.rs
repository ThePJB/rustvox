use crate::chunk::*;
use crate::krand::*;
use crate::settings::*;

/*
lets give this trait thing a try
i wonder if some fancy combinator is possible... sounds slow lol
*/

pub const SEA_LEVEL_F32: f32 = 0.0;
pub const SEA_LEVEL_I32: i32 = 0;

pub trait LevelGenerator: Clone + Send + Sync + 'static {

    fn height(&self, x: f32, z: f32) -> f32;
    fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block>;
}

#[derive(Clone)]
pub struct GenNormalCliffy {
    seed: u32,
}

impl GenNormalCliffy {
    pub fn new(seed: u32) -> GenNormalCliffy {
        GenNormalCliffy {seed}
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
        squished + lf + cliff // good or bad to break da rules, like actual sand squish seems smart
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