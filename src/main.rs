use winit::{
    event::{Event, WindowEvent, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use pixels::{Pixels, SurfaceTexture};
use rand::prelude::*;

use torus::vector::Vector3;
use torus::map::Map;
use torus::perlin::PerlinGenerator;
use torus::raycaster::Raycaster;
use torus::renderer::draw_frame;

fn main() {
    let mut map = Map::new();
    let mut rng = thread_rng();
    println!("Seed: {}", rng.next_u32());
    println!("Generating map...");
    map.generate(&PerlinGenerator::new(rng.next_u32()));
    println!("Map generated!");
    println!("Generating distance maps...");
    map.generate_distance_maps(8);
    println!("Distance maps generated!");

    let raycaster = Raycaster::new(map.clone());
    let mut ray_origin = Vector3::new(0.0, 0.0, 2.0);
    let mut ray_rotation = Vector3::new(0.0, 0.0, 0.0);

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Torus")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height).expect("Error resizing surface");
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        let is_pressed = input.state == ElementState::Pressed;
                        match keycode {
                            VirtualKeyCode::Z => {
                                if is_pressed {
                                    ray_origin.z += 0.1;
                                }
                            }
                            VirtualKeyCode::S => {
                                if is_pressed {
                                    ray_origin.z -= 0.1;
                                }
                            }
                            VirtualKeyCode::A => {
                                if is_pressed {
                                    ray_rotation.y -= 0.1;
                                }
                            }
                            VirtualKeyCode::D => {
                                if is_pressed {
                                    ray_origin.x += 0.1;
                                }
                            }
                            VirtualKeyCode::Q => {
                                if is_pressed {
                                    ray_origin.x -= 0.1;
                                }
                            }
                            VirtualKeyCode::E => {
                                if is_pressed {
                                    ray_rotation.y += 0.1;
                                }
                            }
                            VirtualKeyCode::R => {
                                if is_pressed {
                                    ray_rotation.x -= 0.1;
                                }
                            }
                            VirtualKeyCode::F => {
                                if is_pressed {
                                    ray_rotation.x += 0.1;
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
                draw_frame(&mut pixels, &raycaster, ray_origin, window_size, ray_rotation);
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
        }
    });
}
