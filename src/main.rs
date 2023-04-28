use pixels::{Pixels, SurfaceTexture};
use rand::prelude::*;
use std::collections::HashMap;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use nalgebra::Vector3;
use torus::camera::Camera;
use torus::map::Map;
use torus::perlin::PerlinGenerator;
use torus::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Torus")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    let mut map = Map::new();
    let mut rng = thread_rng();
    println!("Seed: {}", rng.next_u32());
    println!("Generating map...");
    map.generate(&PerlinGenerator::new(rng.next_u32()));
    println!("Map generated!");
    println!("Generating distance maps...");
    map.generate_all_distance_maps(4);

    let renderer = Renderer::new(map.clone(), window_size.width, window_size.height, 8);

    let mut camera = Camera::new(
        renderer,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        0.1,
    );

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(size) => {
                pixels
                    .resize_surface(size.width, size.height)
                    .expect("Error resizing surface");
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::Z => {
                            if input.state == ElementState::Pressed {
                                camera.move_forward();
                            }
                        }
                        VirtualKeyCode::S => {
                            if input.state == ElementState::Pressed {
                                camera.move_backward();
                            }
                        }
                        VirtualKeyCode::Q => {
                            if input.state == ElementState::Pressed {
                                camera.move_left();
                            }
                        }
                        VirtualKeyCode::D => {
                            if input.state == ElementState::Pressed {
                                camera.move_right();
                            }
                        }
                        VirtualKeyCode::R => {
                            if input.state == ElementState::Pressed {
                                camera.move_up();
                            }
                        }
                        VirtualKeyCode::F => {
                            if input.state == ElementState::Pressed {
                                camera.move_down();
                            }
                        }
                        VirtualKeyCode::Up => {
                            if input.state == ElementState::Pressed {
                                camera.rotate_up();
                            }
                        }
                        VirtualKeyCode::Down => {
                            if input.state == ElementState::Pressed {
                                camera.rotate_down();
                            }
                        }
                        VirtualKeyCode::Left => {
                            if input.state == ElementState::Pressed {
                                camera.rotate_left();
                            }
                        }
                        VirtualKeyCode::Right => {
                            if input.state == ElementState::Pressed {
                                camera.rotate_right();
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            let time = std::time::Instant::now();
            camera.draw_frame(&mut pixels);
            println!("Redraw requested");
            println!("FPS: {}", 1.0 / time.elapsed().as_secs_f32());

            if let Err(e) = pixels.render() {
                eprintln!("pixels.render() failed: {:?}", e);
                *control_flow = ControlFlow::Exit;
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
