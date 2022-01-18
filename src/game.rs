use glow::*;

pub struct Game {
    show_menu: bool,
    gl: glow::Context,
    window: glutin::WindowedContext<glutin::PossiblyCurrent>,
    egui: egui_glow::EguiGlow,

    xres: f32,
    yres: f32,

    lock_cursor: bool,
}

unsafe fn opengl_boilerplate(xres: f32, yres: f32) -> (glow::Context, glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("Rustvox")
        .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
    let window = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
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
    gl.clear_color(0.7, 0.7, 0.4, 1.0);
    gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    gl.enable(BLEND);
    gl.debug_message_callback(|a, b, c, d, msg| {
        println!("{} {} {} {} msg: {}", a, b, c, d, msg);
    });

    (gl, window)
}

impl Game {
    pub fn new() -> Game {
        let default_xres = 1600.0;
        let default_yres = 900.0;
        let (gl, window) = unsafe { opengl_boilerplate(default_xres, default_yres) };
        let mut egui_glow = egui_glow::EguiGlow::new(&window, &gl);

        let mut game = Game {
            show_menu: false,
            xres: default_xres,
            yres: default_yres,
            gl,
            window,
            lock_cursor: false,
            egui: egui_glow,
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

    }

    pub fn draw(&mut self) {

    }

    pub fn update(&mut self, dt: f32) {
        
    }


}
