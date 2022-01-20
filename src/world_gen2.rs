use crate::kmath::*;
use crate::krand::*;
use crate::chunk::*;
use crate::settings::*;

pub struct WorldProps2D {
    height: f32,
    grad: Vec2,
}

#[derive(Clone, Copy)]
pub struct WorldGen {
    seed: u32,
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

impl WorldGen {
    pub fn new(seed: u32) -> WorldGen {
        WorldGen { seed }
    }

    pub fn height(&self, x: f32, z: f32) -> f32 {
        let p = 0.005 * Vec2 { x, y: z};

        let p_lf = 0.0005 * Vec2{x, y:z};
        let h_lf = fbm1(p_lf + 0.5*fbm2(p, self.seed + 0x12345131), self.seed + 0x42141213);

        let h_mountain = fbm1(p + fbm2(p + fbm2(p + fbm2(p, self.seed + 0x31261343), self.seed + 0x91376513), self.seed + 0x23452337), self.seed);
        let h_mountain_sharp = h_mountain * h_mountain * h_mountain * h_mountain * 5.0;

        let h_mountain = h_mountain.max(h_mountain_sharp); // or softmax

        let p_rough = 0.002 * Vec2 { x, y: z};
        let roughness = fbm1(p_rough, self.seed + 34192313);
        let t_mountain = saturate(roughness - 0.1, 0.35, 0.65);

        let h_land = h_lf + t_mountain * t_mountain * h_mountain;
        

        let h_ocean = 0.1;
        let t_ocean = saturate(fgrad2_isotropic(0.0005 * x, 0.0005 * z, self.seed + 34111231), 0.50, 0.7);

        let h = lerp(h_land, h_ocean, t_ocean);
        (h - 0.4) * 200.0
    }

    pub fn props_2d(&self, x: f32, z: f32) -> WorldProps2D {
        let height = self.height(x,z);
        let hgx = self.height(x + 1.0, z + 0.0);
        let hgz = self.height(x + 0.0, z + 1.0);

        let gradx = height - hgx;
        let gradz = height - hgz;
        let grad = Vec2::new(gradx, gradz);

        WorldProps2D {
            height,
            grad,
        }
    }

    pub fn generate_blocks(&self, ox: i32, oy: i32, oz: i32) -> Vec<Block> {
        let mut blocks = vec![Block::Air; S*S*S];
        for k in 0..S {
            let z = oz*S as i32 + k as i32;
    
            for i in 0..S {
                let x = ox*S as i32 + i as i32;
                let props = self.props_2d(x as f32, z as f32);
                let height = props.height as i32;
                let grad = props.grad;
            
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