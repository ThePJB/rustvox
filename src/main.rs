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
mod kimg;
mod game;
mod world_gen2;

use kimg::*;
use glow::*;
use glam::{Vec3, Mat4};
use std::error::Error;
use std::time::{Duration, SystemTime};
use std::collections::HashSet;
use std::f32::consts::PI;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use chunk::*;
use chunk_manager::*;
use elemesh::*;
use world_gen::*;
use settings::*;
use camera::*;
use game::*;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut game = Game::new(&event_loop);
    let mut held_keys: HashSet<glutin::event::VirtualKeyCode> = HashSet::new();
    let mut dt = 1.0f64 / 60.0f64;

    let mut prev_frame_start = SystemTime::now();
    let mut curr_frame_start = SystemTime::now();

    let mut frame = 0;

    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        let mut cleanup = || {
            // game.destroy(); lol borrowing. maybe I could implement drop for game, maybe that will get called
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
                game.update(&held_keys, dt as f32);
                game.draw();
                // game.window.window().set_title(&format!("RustVox | {:.2}ms", dt*1000.0));
            }

            Event::DeviceEvent {device_id: _, event: glutin::event::DeviceEvent::Motion {axis, value}} => {
                if axis == 0 {
                    game.look(value as f32, 0.0);
                } else {
                    game.look(0.0, value as f32);
                }
            },

            Event::WindowEvent { ref event, .. } =>{
                game.egui_event(&event);

                match event {
                    WindowEvent::Resized(physical_size) => {
                        game.resize(physical_size.width as f32, physical_size.height as f32);
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
                            (glutin::event::VirtualKeyCode::Escape, glutin::event::ElementState::Released) => {
                                cleanup();
                            },
                            (glutin::event::VirtualKeyCode::F2, glutin::event::ElementState::Released) => {
                                game.screenshot();
                                // screenshot(&gl, window_x as usize, window_y as usize);
                            },
                            (glutin::event::VirtualKeyCode::F3, glutin::event::ElementState::Released) => {
                                game.toggle_menu();
                            },
                        _ => (),
                    }},
                    _ => (),
                }
            },
            _ => (),
        }
    });
}