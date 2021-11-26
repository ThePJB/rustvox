use glow::*;

pub struct Elemesh {
    vao: glow::NativeVertexArray,
    vbo: glow::NativeBuffer,
    ebo: glow::NativeBuffer,
    num_verts:  i32,
}

impl Elemesh {
    pub fn new(gl: &glow::Context, vertex_data: Vec<f32>, index_data: Vec<u16>) -> Elemesh {
        let num_verts = index_data.len() as i32;

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

            let vertex_u8 = {
                let (ptr, len, cap) = vertex_data.into_raw_parts();
                Vec::from_raw_parts(ptr as *mut u8, len*4, cap*4)
            };
            let element_u8 = {
                let (ptr, len, cap) = index_data.into_raw_parts();
                Vec::from_raw_parts(ptr as *mut u8, len*2, cap*2)
            };
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertex_u8, glow::STATIC_DRAW);
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, &element_u8, glow::STATIC_DRAW);

            (vao, vbo, ebo)
        };

        Elemesh {
            vao,
            vbo,
            ebo,
            num_verts,
        }

    }

    pub fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            gl.draw_elements(glow::TRIANGLES, self.num_verts, glow::UNSIGNED_SHORT, 0);
        }
    }
}