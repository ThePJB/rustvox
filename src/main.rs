#![feature(vec_into_raw_parts)]
mod chunk;
mod chunk_manager;
mod elemesh;
mod kmath;
mod krand;
mod priority_queue;
mod world_gen;
mod settings;
mod camera;

use glow::*;
use glam::{Vec3, Mat4};
use std::error::Error;
use std::time::{Duration, SystemTime};
use std::collections::HashSet;
use std::f32::consts::PI;

use chunk::*;
use chunk_manager::*;
use elemesh::*;
use world_gen::*;
use settings::*;
use camera::*;

/*
Coordinate system:

+Z into screen
-X left +X right
-Y down +Y up

LH

*/


fn main() -> Result<(), Box<dyn Error>> {

    let mut window_x = 1600.0;
    let mut window_y = 900.0;

    let fovx = 0.9;
    let a = window_x / window_y;

    // let proj = Mat4::perspective_lh(fovx, 16.0/9.0, 0.01, 1000.0);


    // println!("proj: {}", proj);

    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Rustvox")
            .with_inner_size(glutin::dpi::PhysicalSize::new(window_x, window_y));
        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();
        window.window().set_cursor_grab(true)?;
        window.window().set_cursor_visible(false);

        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(DEPTH_TEST);
        gl.enable(CULL_FACE);
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
        gl.enable(BLEND);
        gl.debug_message_callback(|a, b, c, d, msg| {
            println!("{} {} {} {} msg: {}", a, b, c, d, msg);
        });

        let program_pc = gl.create_program().expect("Cannot create program");

        {   // Shader stuff
            let shader_version = "#version 410";
            let shader_sources = [
                (glow::VERTEX_SHADER, std::fs::read_to_string("src/test.vert")?),
                (glow::FRAGMENT_SHADER, std::fs::read_to_string("src/test.frag")?),
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
                gl.attach_shader(program_pc, shader);
                shaders.push(shader);
            }
            gl.link_program(program_pc);
            if !gl.get_program_link_status(program_pc) {
                panic!("{}", gl.get_program_info_log(program_pc));
            }
            for shader in shaders {
                gl.detach_shader(program_pc, shader);
                gl.delete_shader(shader);
            }
        }

        let program_pcn = gl.create_program().expect("Cannot create program");

        {   // Shader stuff
            let shader_version = "#version 410";
            let shader_sources = [
                (glow::VERTEX_SHADER, std::fs::read_to_string("src/pcn.vert")?),
                (glow::FRAGMENT_SHADER, std::fs::read_to_string("src/pcn.frag")?),
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
                gl.attach_shader(program_pcn, shader);
                shaders.push(shader);
            }
            gl.link_program(program_pcn);
            if !gl.get_program_link_status(program_pcn) {
                panic!("{}", gl.get_program_info_log(program_pcn));
            }
            for shader in shaders {
                gl.detach_shader(program_pcn, shader);
                gl.delete_shader(shader);
            }
            gl.use_program(Some(program_pcn));
        }


        // game stuff
        // let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u32;
        let seed = 0;
        let gen = GenWarp::new(seed);

        let mut chunk_manager = ChunkManager::new(&gl, &gen);

        let test_cube_vertex_buffer = vec![
            -1.0f32, 1.0, 1.0,
            1.0, 0.0, 0.0,

            -1.0, -1.0, 1.0,
            0.0, 1.0, 0.0,

            1.0, -1.0, 1.0,
            0.0, 0.0, 1.0,

            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,


            -1.0, 1.0, -1.0,
            1.0, 0.0, 0.0,

            -1.0, -1.0, -1.0,
            0.0, 1.0, 0.0,

            1.0, -1.0, -1.0,
            0.0, 0.0, 1.0,

            1.0, 1.0, -1.0,
            1.0, 1.0, 1.0,



            -1.0, -1.0, 1.0,
            1.0, 0.0, 0.0,

            -1.0, -1.0, -1.0,
            0.0, 1.0, 0.0,

            1.0, -1.0, -1.0,
            0.0, 0.0, 1.0,

            1.0, -1.0, 1.0,
            1.0, 1.0, 1.0,


            -1.0, 1.0, 1.0,
            1.0, 0.0, 0.0,

            -1.0, 1.0, -1.0,
            0.0, 1.0, 0.0,

            1.0, 1.0, -1.0,
            0.0, 0.0, 1.0,

            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,


            1.0, 1.0, -1.0,
            1.0, 0.0, 0.0,

            1.0, -1.0, -1.0,
            0.0, 1.0, 0.0,

            1.0, -1.0, 1.0,
            0.0, 0.0, 1.0,

            1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,


            -1.0, 1.0, -1.0,
            1.0, 0.0, 0.0,

            -1.0, -1.0, -1.0,
            0.0, 1.0, 0.0,

            -1.0, -1.0, 1.0,
            0.0, 0.0, 1.0,

            -1.0, 1.0, 1.0,
            1.0, 1.0, 1.0,


        ];


        let test_cube_index_buffer = vec![
            0u16, 1, 2, 0, 2, 3,

            4, 5, 6, 4, 6, 7,

            8, 9, 10, 8, 10, 11,

            12, 13, 14, 12, 14, 15,

            16, 17, 18, 16, 18, 19,

            20, 21, 22, 20, 22, 23,
        ];

        let cube = Elemesh::new(&gl, test_cube_vertex_buffer, test_cube_index_buffer);

        let plane_s = 1000.0;
        let plane_h = 1.0;

        // sky plane void plane
        let plane_verts = vec![
            plane_s, plane_h, plane_s,
            0.4, 0.4, 0.7,

            plane_s, plane_h, -plane_s,
            0.4, 0.4, 0.7,


            -plane_s, plane_h, -plane_s,
            0.4, 0.4, 0.7,

            -plane_s, plane_h, plane_s,
            0.4, 0.4, 0.7,


            -plane_s, -plane_h, -plane_s,
            0.0, 0.0, 0.5,

            plane_s, -plane_h, -plane_s,
            0.0, 0.0, 0.5,

            plane_s, -plane_h, plane_s,
            0.0, 0.0, 0.5,

            -plane_s, -plane_h, plane_s,
            0.0, 0.0, 0.5,
        ];

        let plane_idxs = vec![
            0u16, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
        ];

        let plane = Elemesh::new(&gl, plane_verts, plane_idxs);

        let mut cam = Camera::new(fovx, a, kmath::Vec3::new(0.0, gen.height(0.0, 0.0) + 3.0, 0.0));
        // handle those resize events and update it too

        let mut held_keys: HashSet<glutin::event::VirtualKeyCode> = HashSet::new();
        let mut dt = 1.0f64 / 60f64;

        use glutin::event::{Event, WindowEvent};
        use glutin::event_loop::ControlFlow;

        let mut frame = 0;

        let mut prev_frame_start = SystemTime::now();
        let mut curr_frame_start = SystemTime::now();


        event_loop.run(move |event, _, control_flow| {


            *control_flow = ControlFlow::Poll;

            let mut cleanup = || {
                gl.delete_program(program_pc);
                gl.delete_program(program_pcn);
                *control_flow = ControlFlow::Exit;
            };

            match event {
                Event::LoopDestroyed |
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} |
                Event::WindowEvent {event: WindowEvent::KeyboardInput {
                    input: glutin::event::KeyboardInput { virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape), ..}, ..}, ..}
                => {
                    cleanup();
                },

                Event::NewEvents(_) => {
                    prev_frame_start = curr_frame_start;
                    curr_frame_start = SystemTime::now();
                    dt = curr_frame_start.duration_since(prev_frame_start).unwrap().as_secs_f64();
                    frame += 1;
                }

                Event::MainEventsCleared => {
                    // update
                    let update_start = SystemTime::now();

                    window.window().set_cursor_position(glutin::dpi::PhysicalPosition::new(window_x as i32, window_y as i32)).unwrap();

                    let speed = 128.0f32;
                    if held_keys.contains(&glutin::event::VirtualKeyCode::W) {
                        cam.update_z(speed*dt as f32);
                    }
                    if held_keys.contains(&glutin::event::VirtualKeyCode::S) {
                        cam.update_z(-speed*dt as f32);
                    }
                    if held_keys.contains(&glutin::event::VirtualKeyCode::A) {
                        cam.update_x(-speed*dt as f32);
                    }
                    if held_keys.contains(&glutin::event::VirtualKeyCode::D) {
                        cam.update_x(speed*dt as f32);
                    }
                    if held_keys.contains(&glutin::event::VirtualKeyCode::Space) {
                        cam.update_y(speed*dt as f32);
                    }
                    if held_keys.contains(&glutin::event::VirtualKeyCode::LShift) {
                        cam.update_y(-speed*dt as f32);
                    }

                    let treadmill = SystemTime::now();


                    chunk_manager.treadmill(&gl, &cam, &gen);

                    let draw = SystemTime::now();
                    // draw
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    gl.use_program(Some(program_pc));

                    let proj = cam.projection_mat();

                    gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program_pc, "projection").as_ref(),
                    false, &proj.to_cols_array());

                    let view_planes = cam.view_mat_nomove();
                    gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program_pc, "view").as_ref(),
                    false, &view_planes.to_cols_array());
                    
                    plane.draw(&gl);
                    gl.clear(glow::DEPTH_BUFFER_BIT);
                    
                    let view = cam.view_mat();
                    gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program_pc, "view").as_ref(),
                        false, &view.to_cols_array());

                    cube.draw(&gl);
                    

                    gl.use_program(Some(program_pcn));
                    gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program_pcn, "projection").as_ref(),
                    false, &proj.to_cols_array());
                    gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(program_pcn, "view").as_ref(),
                        false, &view.to_cols_array());

                    chunk_manager.draw(&gl, &cam);
                    
                    let finish_draw = SystemTime::now();
                    
                    window.swap_buffers().unwrap();
                    
                    let finish_swap_buffers = SystemTime::now();

                    let t_events = update_start.duration_since(curr_frame_start).unwrap().as_secs_f64();
                    let t_update = treadmill.duration_since(update_start).unwrap().as_secs_f64();
                    let t_treadmill = draw.duration_since(treadmill).unwrap().as_secs_f64();
                    let t_draw = finish_draw.duration_since(draw).unwrap().as_secs_f64();
                    let t_swap = finish_swap_buffers.duration_since(finish_draw).unwrap().as_secs_f64();

                    let (omesh, tmesh) = chunk_manager.chunk_map.iter().map(|(key, val)| (if val.opaque_mesh.is_some() {
                        1
                    } else {
                        0
                    }, if val.transparent_mesh.is_some() {
                        1
                    } else {
                        0
                    })).fold((0,0), |(ao, at), (o, t)| (ao + o, at + t));

                    let (otri, ttri) = chunk_manager.chunk_map.iter().map(|(key, val)| (if val.opaque_mesh.is_some() {
                        val.opaque_mesh.as_ref().unwrap().num_triangles
                    } else {
                        0
                    }, if val.transparent_mesh.is_some() {
                        val.transparent_mesh.as_ref().unwrap().num_triangles
                    } else {
                        0
                    })).fold((0,0), |(ao, at), (o, t)| (ao + o, at + t));

                    println!("events: {:.2} update: {:.2} treadmill: {:.2}, draw: {:.2} swap: {:.2} omesh: {} kotri: {} tmesh: {} kttri:{}", t_events*1000.0, t_update*1000.0, t_treadmill*1000.0, t_draw*1000.0, t_swap*1000.0, omesh, otri/1000, tmesh, ttri/1000);
                    window.window().set_title(&format!("RustVox | {:.2}ms", dt*1000.0));
                }

                Event::DeviceEvent {device_id: _, event: glutin::event::DeviceEvent::Motion {axis, value}} => {
                    if axis == 0 {
                        cam.update_look(value as f32, 0.0);
                    } else {
                        cam.update_look(0.0, value as f32);
                    }
                },


                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                        window_x = physical_size.width as f32;
                        window_y = physical_size.height as f32;
                        gl.viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                        println!("aspect ratio: {:?}", window_x / window_y);
                        // prob update aspect  ratio in camera

                    }
                    WindowEvent::CloseRequested => {
                        cleanup();
                    }
                    WindowEvent::KeyboardInput {
                        input: glutin::event::KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                        ..
                    } => {
                        match state {
                            glutin::event::ElementState::Pressed => held_keys.insert(*virtual_code),
                            glutin::event::ElementState::Released => held_keys.remove(virtual_code),
                        };

                        match (virtual_code, state) {
                            (glutin::event::VirtualKeyCode::Escape, _) => {
                                cleanup();
                            },
                        _ => (),
                    }},
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
