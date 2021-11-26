pub fn khash(seed: u32) -> u32 {
    let n1 = 0xB5297A4D;
    let n2 = 0x68E31DA4;
    let n3 = 0x1B56C4E9;

    let mut mangled = seed;
    mangled = mangled.wrapping_mul(n1);
    mangled ^= mangled.rotate_right(13);
    mangled = mangled.wrapping_add(n2);
    mangled ^= mangled.rotate_left(7);
    mangled = mangled.wrapping_mul(n3);
    mangled ^= mangled.rotate_right(9);
    return mangled;
}

// 0..1
fn f322(x: u32, y: u32, seed: u32) -> f32 {
    khash(x + y * 0xA341316C + seed * 0xF73DB187) as f32 / std::u32::MAX as f32
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0-t) + b * t
}

fn bilinear(a: f32, b: f32, c: f32, d: f32, t1: f32, t2: f32) -> f32 {
    //let u = |x| x*x*(3.0-2.0*x);
    //let u = |x| x*x*(10.0-3.0*x*(5.0-2.0*x));                 // looks fucked, directional artifacts I wonder why. maybe because derivative is discontinuous in middle
    //let u = |x: f32| (std::f32::consts::FRAC_PI_2*x).sin();   // looks fucked, I expected better from you sin, maybe derivative discontinuous in middle
    //let u = |x| x;
    let u = |x| ((6.0*x - 15.0)*x + 10.0)*x*x*x;
    lerp(lerp(a, b, u(t1)), lerp(c, d, u(t1)), u(t2))
}

const ROOT3ON2: f32 = 0.8660254037844386467637231707529361834714026269051903140279034897;

pub fn grad2_isotropic(x: f32, y: f32, seed: u32) -> f32 {
    let root2 = std::f32::consts::SQRT_2;

    // also why not use a bigger gradient table
    //let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (root2, root2), (-root2, root2), (root2, -root2), (-root2, -root2)];
    let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (root2, root2), (-root2, root2), (root2, -root2), (-root2, -root2),
        (0.5, ROOT3ON2), (0.5, ROOT3ON2), (-0.5, -ROOT3ON2), (-ROOT3ON2, -0.5), (-0.5, ROOT3ON2), (-ROOT3ON2, 0.5), (0.5, -ROOT3ON2), (ROOT3ON2, -0.5)
    ];
    // idk whystefan gustavson does the below and not the above. it kinda does look better lol
    // also why not more gradients?
    //let grads = [(-1.0, 0.0), (1.0, 0.0), (0.0, 1.0), (0.0, -1.0), (1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)];

    let xu = x.floor() as u32;
    let yu = y.floor() as u32;

    let cf = |corner_x: u32, corner_y: u32| {
        let g_idx = khash(xu + corner_x + (yu + corner_y) * 0xA341316C + seed * 0xF73DB187) & 15;
        let (dx, dy) = grads[g_idx as usize];
        dx * (x.fract() - corner_x as f32) + dy * (y.fract() - corner_y as f32)
    };
    
    let c1 = cf(0,0);
    let c2 = cf(1,0);
    let c3 = cf(0,1);
    let c4 = cf(1,1);

    let result = bilinear(c1, c2, c3, c4, x.fract(), y.fract());
    
        if x == 1.5 && y == 1.5 {
            println!("c1 {} c2 {} c3 {} c4 {}", c1, c2, c3, c4);
            println!("res: {}", result);
        }
    (result + 1.0) / 2.0
}

pub fn fgrad2_isotropic(x: f32, y: f32, seed: u32) -> f32 {
    (1.000 * grad2_isotropic(x, y, seed*0x3523423) +
    0.500 * grad2_isotropic(x * 2.0, y * 2.0, seed*0xF73DB187) + 
    0.250 * grad2_isotropic(x * 4.0, y * 4.0, seed*0x159CBAFE) + 
    0.125 * grad2_isotropic(x * 8.0, y * 8.0, seed*0x83242364)) /
    1.675
}