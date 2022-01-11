use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use crate::kmath::*;

pub struct ImageBuffer {
    pub w: usize,
    pub h: usize,
    pub pixels: Vec<(u8,u8,u8)>,
}

// should actually just use a vec u8 internally so no need to convert back and forth

impl ImageBuffer {
    pub fn new(w: usize, h: usize) -> ImageBuffer {
        ImageBuffer {
            w,
            h,
            pixels: vec![(0,0,0); w*h],
        }
    }
    pub fn set_px(&mut self, x: usize, y: usize, val: (u8, u8, u8)) {
        self.pixels[y*self.w + x] = val;
    }
    pub fn get_px(&self, x: usize, y: usize) -> (u8, u8, u8) {
        self.pixels[y*self.w + x]
    }
    
    pub fn new_from_file(path_str: &str) -> ImageBuffer {
        let decoder = png::Decoder::new(File::open(path_str).unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.x: usize
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let bytes = &buf[..info.buffer_size()];
        let mut bytes_idx = 0;
        // extra copy whatever idgaf
        let mut image_buffer = ImageBuffer::new(info.width as usize, info.height as usize);
        for j in 0..image_buffer.h {
            for i in 0..image_buffer.w {
                image_buffer.set_px(i, j, (bytes[bytes_idx], bytes[bytes_idx + 1], bytes[bytes_idx + 2]));
                bytes_idx += 3;
            }
        }
        image_buffer
    }
    pub fn dump_to_file(&self, path_str: &str) {
        let path = Path::new(path_str);
        let file = File::create(path).unwrap();
        let ref mut buf_writer = BufWriter::new(file);
    
        let mut data = vec![0u8; (3*self.w*self.h)];
        let mut data_index = 0;
        for px in self.pixels.iter() {
            data[data_index] = px.0;
            data_index += 1;
            data[data_index] = px.1;
            data_index += 1;
            data[data_index] = px.2;
            data_index += 1;
        }
    
        let mut encoder = png::Encoder::new(buf_writer, self.w as u32, self.h as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8));
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&data).unwrap(); // Save
    }
}

pub fn dump_image_fn(
        path_str: &str, 
        f: &dyn Fn(f32, f32) -> f32, 
        c: &dyn Fn(f32) -> Vec3, 
        x_px: usize, x_start: f32, x_end: f32, 
        y_px: usize, y_start: f32, y_end: f32) {

    let mut imbuf = ImageBuffer::new(x_px, y_px);

    for i in 0..x_px {
        for j in 0..y_px {
            let xt = i as f32 / x_px as f32;
            let yt = j as f32 / y_px as f32;

            let x = lerp(x_start, x_end, xt);
            let y = lerp(y_start, y_end, yt);

            let colour = c(f(x, y));
            let convert = |c: Vec3| ((255.0 * c.x) as u8, (255.0 * c.y) as u8, (255.0 * c.z) as u8);            
            imbuf.set_px(i, j, convert(colour));
        }
    }

    imbuf.dump_to_file(path_str);
}

#[test]
fn test_im_fn() {
    dump_image_fn("test.png", 
    &|x, y| x*y,
    &|c| Vec3::new(c,c,c),
    100, 0.0, 1.0,
    100, 0.0, 1.0);
}