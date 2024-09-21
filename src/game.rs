use glow::*;
use crate::chunk_manager::*;
use crate::world_gen::*;
use crate::world_gen2::*;
use crate::camera::*;
use crate::kmath::*;
use std::collections::HashSet;
use crate::kimg::*;

pub struct Game {
    show_menu: bool,
    gl: glow::Context,
    window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    egui: egui_glow::EguiGlow,

    xres: f32,
    yres: f32,

    pub lock_cursor: bool,

    pc_program: glow::Program,
    pcn_program: glow::Program,

    chunk_manager: ChunkManager,
    cam: Camera,

    fog_intensity: f32,
    fog_colour: [f32; 3],
}

fn  make_shader(gl: &glow::Context, vert_path: &str, frag_path: &str) -> glow::Program {
    unsafe {
        let program = gl.create_program().expect("Cannot create program");
        let shader_version = "#version 410";
        let shader_sources = [
            (glow::VERTEX_SHADER, std::fs::read_to_string(vert_path).unwrap()),
            (glow::FRAGMENT_SHADER, std::fs::read_to_string(frag_path).unwrap()),
            ];
        let mut shaders = Vec::with_capacity(shader_sources.len());
        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
            .create_shader(*shader_type)
            .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        
        program
    }
}

unsafe fn opengl_boilerplate(xres: f32, yres: f32, event_loop: &glutin::event_loop::EventLoop<()>) -> (glow::Context, glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Rustvox")
        .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
    let window = glutin::ContextBuilder::new()
        // .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true)
        .build_windowed(window_builder, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();


    let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    gl.enable(DEPTH_TEST);
    gl.enable(CULL_FACE);
    gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    gl.enable(BLEND);
    gl.debug_message_callback(|a, b, c, d, msg| {
        println!("{} {} {} {} msg: {}", a, b, c, d, msg);
    });

    (gl, window)
}

impl Game {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Game {
        let default_xres = 1600.0;
        let default_yres = 900.0;
        let fovx = 0.9;

        let (gl, window) = unsafe { opengl_boilerplate(default_xres, default_yres, event_loop) };
        let egui_glow = egui_glow::EguiGlow::new(&window, &gl);

        let pc_program = make_shader(&gl, "src/test.vert", "src/test.frag");
        let pcn_program = make_shader(&gl, "src/pcn.vert", "src/pcn.frag");

        let gen = WorldGen::new(69);
        let chunk_manager = ChunkManager::new(&gl, gen);

        let cam = Camera::new(fovx, default_xres/default_yres, Vec3::new(0.0, gen.height(0.0, 0.0) + 1.0, 0.0));

        let mut game = Game {
            show_menu: false,
            xres: default_xres,
            yres: default_yres,
            gl,
            window,
            lock_cursor: false,
            egui: egui_glow,
            pc_program,
            pcn_program,
            chunk_manager,
            cam,
            fog_intensity: 0.0003,
            fog_colour: [0.0, 0.0, 0.0],
        };

        game.lock_focus();

        game
    }

    pub fn lock_focus(&mut self) {
        self.window.window().set_cursor_grab(true).unwrap();
        self.window.window().set_cursor_visible(false);
        self.lock_cursor = true;
    }

    pub fn unlock_focus(&mut self) {
        self.window.window().set_cursor_grab(false).unwrap();
        self.window.window().set_cursor_visible(true);
        self.lock_cursor = false;
    }

    pub fn resize(&mut self, new_xres: f32, new_yres: f32) {
        let ps = winit::dpi::PhysicalSize::new(new_xres as u32, new_yres as u32);
        self.window.resize(ps);
        self.xres = new_xres;
        self.yres = new_yres;
        unsafe { self.gl.viewport(0, 0, new_xres as i32, new_yres as i32) };
        // projection mat aspect ratio too?
    }

    pub fn screenshot(&self) {
        let w = self.xres as usize;
        let h = self.yres as usize;

        // gl read pixels etc
        let size = (w*h*4);
        let mut buf = vec![0; size];
        let ppd = PixelPackData::Slice(&mut buf);
        unsafe {
            self.gl.read_pixels(0, 0, self.xres as i32, self.yres as i32, RGBA, UNSIGNED_BYTE, ppd);
        }
        let mut imbuf = ImageBuffer::new(self.xres as usize, self.yres as usize);
        // y is flipped colours are cooked
        for i in 0..w {
            for j in 0..h {
                let colour = (
                    buf[4 * i +  4 * (h-j-1) * w + 0],
                    buf[4 * i +  4 * (h-j-1) * w + 1],
                    buf[4 * i +  4 * (h-j-1) * w + 2],
                );
    
                imbuf.set_px(i, j, colour);
            }
        }
        imbuf.dump_to_file("screen.png");
    }
    

    pub fn look(&mut self, x: f32, y: f32) {
        if self.lock_cursor {
            self.cam.update_look(x, y);
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            let proj = self.cam.projection_mat();
            let view = self.cam.view_mat();
            
            self.gl.clear_color(self.fog_colour[0], self.fog_colour[1], self.fog_colour[2], 1.0);
            self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            
            self.gl.use_program(Some(self.pcn_program));
            self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.pcn_program, "projection").as_ref(),
            false, &proj.to_cols_array());
            self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.pcn_program, "view").as_ref(),
            false, &view.to_cols_array());
            self.gl.uniform_3_f32(self.gl.get_uniform_location(self.pcn_program, "cam_pos").as_ref(), self.cam.pos.x, self.cam.pos.y, self.cam.pos.z);
            self.gl.uniform_3_f32(self.gl.get_uniform_location(self.pcn_program, "cam_dir").as_ref(), self.cam.dir.x, self.cam.dir.y, self.cam.dir.z);
            self.gl.uniform_3_f32(self.gl.get_uniform_location(self.pcn_program, "fog_colour").as_ref(), self.fog_colour[0], self.fog_colour[1], self.fog_colour[2]);
            self.gl.uniform_1_f32(self.gl.get_uniform_location(self.pcn_program, "fog_intensity").as_ref(), self.fog_intensity);
            
            self.chunk_manager.draw(&self.gl, &self.cam);
            
            if self.show_menu {
                self.gl.disable(DEPTH_TEST);
                
                
                let (needs_repaint, shapes) = self.egui.run(self.window.window(), |egui_ctx| {
                    egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                        ui.heading("Fog Intensity");
                        ui.add(egui::Slider::new(&mut self.fog_intensity, 0.0..=0.01));
                        ui.end_row();

                        ui.heading("Fog Colour");
                        ui.color_edit_button_rgb(&mut self.fog_colour);
                        ui.end_row();

                        ui.heading("Hello World!");
                        if ui.button("Quit").clicked() {
                            println!("spaget");
                        }
                    });
                });
        
                self.egui.paint(&self.window, &self.gl, shapes);
                self.gl.enable(DEPTH_TEST);
            }
            
            self.window.swap_buffers().unwrap();
        }
    }

    pub fn update(&mut self, held_keys: &HashSet<glutin::event::VirtualKeyCode>, dt: f32) {

        if self.lock_cursor {
            self.window.window().set_cursor_position(glutin::dpi::PhysicalPosition::new((self.xres/2.0) as i32, (self.yres/2.0) as i32)).unwrap();
        }

        let speed = 32.0f32;
        if held_keys.contains(&glutin::event::VirtualKeyCode::W) {
            self.cam.update_z(speed*dt as f32);
        }
        if held_keys.contains(&glutin::event::VirtualKeyCode::S) {
            self.cam.update_z(-speed*dt as f32);
        }
        if held_keys.contains(&glutin::event::VirtualKeyCode::A) {
            self.cam.update_x(-speed*dt as f32);
        }
        if held_keys.contains(&glutin::event::VirtualKeyCode::D) {
            self.cam.update_x(speed*dt as f32);
        }
        if held_keys.contains(&glutin::event::VirtualKeyCode::Space) {
            self.cam.update_y(speed*dt as f32);
        }
        if held_keys.contains(&glutin::event::VirtualKeyCode::LShift) {
            self.cam.update_y(-speed*dt as f32);
        }


        self.chunk_manager.treadmill(&self.gl, &self.cam);

    }

    pub fn destroy(&mut self) {
        unsafe {
            self.gl.delete_program(self.pc_program);
            self.gl.delete_program(self.pcn_program);
        }
    }

    pub fn egui_event(&mut self, event: &glutin::event::WindowEvent) {
        self.egui.on_event(event);
    }

    pub fn toggle_menu(&mut self) {
        if self.show_menu {
            self.lock_focus();
            self.show_menu = false;
        } else {
            self.unlock_focus();
            self.show_menu = true;
        }
    }
}

