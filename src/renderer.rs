use crate::map::Map;
use crate::voxel::Voxel;
use nalgebra::Vector3;
use pixels::Pixels;
use rayon::scope;

pub struct Renderer {
    pub map: Map,
    pub width: u32,
    pub height: u32,
    pub num_threads: usize,
}

impl Renderer {
    pub fn new(map: Map, width: u32, height: u32, num_threads: usize) -> Self {
        Self {
            map,
            width,
            height,
            num_threads,
        }
    }

    fn calc_ray_direction(
        &self,
        x: u32,
        y: u32,
        fov: f32,
        aspect_ratio: f32,
        rotation_angle: Vector3<f32>,
    ) -> Vector3<f32> {
        let ndc_x = (x as f32 + 0.5) / self.width as f32 * 2.0 - 1.0;
        let ndc_y = -((y as f32 + 0.5) / self.height as f32 * 2.0 - 1.0);

        let ray_direction = Vector3::new(
            ndc_x * aspect_ratio * (fov / 2.0).tan(),
            ndc_y * (fov / 2.0).tan(),
            1.0,
        );

        let rotation = nalgebra::Rotation3::from_euler_angles(
            rotation_angle.x,
            rotation_angle.y,
            rotation_angle.z,
        );
        rotation * ray_direction
    }

    fn set_pixel_color(&self, frame: &mut [u8], x: u32, y: u32, width: u32, color: Vector3<u8>) {
        let index = (x + y * width) as usize;

        frame[index * 4] = color.x;
        frame[index * 4 + 1] = color.y;
        frame[index * 4 + 2] = color.z;
        frame[index * 4 + 3] = 255;
    }

    pub fn draw_frame(
        &self,
        pixels: &mut Pixels,
        ray_origin: Vector3<f32>,
        rotation_angle: Vector3<f32>,
    ) {
        let frame = pixels.frame_mut();
        let (width, height) = (self.width, self.height);
        let fov: f32 = 60.0_f32.to_radians();
        let aspect_ratio = width as f32 / height as f32;

        let frame = std::sync::Mutex::new(frame);

        scope(|s| {
            for i in 0..self.num_threads {
                let frame = &frame;
                let ray_origin = ray_origin.clone();
                let rotation_angle = rotation_angle.clone();

                s.spawn(move |_| {
                    for y in (i as u32..height).step_by(self.num_threads) {
                        for x in 0..width {
                            let ray_direction = self.calc_ray_direction(x, y, fov, aspect_ratio, rotation_angle).normalize();

                            let color = {
                                if let Some(intersected_voxel) = self.dda(&ray_origin, &ray_direction, 256).1 {
                                    intersected_voxel.color
                                } else {
                                    Vector3::zeros()
                                }
                            };

                            let mut frame = frame.lock().unwrap();
                            self.set_pixel_color(&mut frame, x, y, width, color);
                        }
                    }
                });
            }
        });
    }

    pub fn dda(
        &self,
        ray_origin: &Vector3<f32>,
        ray_direction: &Vector3<f32>,
        max_step: i32,
    ) -> (i32, Option<&Voxel>) {
        let mut grid_pos = ray_origin.map(|v| v.floor() as f32);
        let grid_step = ray_direction.map(|v| v.signum() as f32);
        let mut t_max = (grid_pos.zip_map(&grid_step, |v, s| v + s as f32) - ray_origin)
            .component_div(ray_direction)
            .map(|v| v.abs());
        let t_delta = grid_step.component_div(ray_direction).map(|v| v.abs());

        let mut step_count = 0;
        while step_count < max_step {
            if !self
                .map
                .is_within_bounds(grid_pos.x as i32, grid_pos.y as i32, grid_pos.z as i32)
            {
                break;
            }

            let distance =
                self.map
                    .get_distance(grid_pos.x as i32, grid_pos.y as i32, grid_pos.z as i32)
                    as f32;
            if distance > 1.0 {
                let step = (distance / ray_direction.magnitude()).ceil() as i32;
                for _ in 0..step {
                    let step_axis = t_max.imin();
                    match step_axis {
                        0 => {
                            t_max.x += t_delta.x;
                            grid_pos.x += grid_step.x;
                        }
                        1 => {
                            t_max.y += t_delta.y;
                            grid_pos.y += grid_step.y;
                        }
                        2 => {
                            t_max.z += t_delta.z;
                            grid_pos.z += grid_step.z;
                        }
                        _ => unreachable!(),
                    }
                    step_count += 1;
                    if step_count >= max_step {
                        break;
                    }
                }
            } else {
                if let Some(voxel) =
                    self.map
                        .get_voxel(grid_pos.x as i32, grid_pos.y as i32, grid_pos.z as i32)
                {
                    if !voxel.is_empty {
                        return (step_count, Some(voxel));
                    }
                }

                let step_axis = t_max.imin();
                match step_axis {
                    0 => {
                        t_max.x += t_delta.x;
                        grid_pos.x += grid_step.x;
                    }
                    1 => {
                        t_max.y += t_delta.y;
                        grid_pos.y += grid_step.y;
                    }
                    2 => {
                        t_max.z += t_delta.z;
                        grid_pos.z += grid_step.z;
                    }
                    _ => unreachable!(),
                }

                step_count += 1;
            }
        }

        (step_count, None)
    }
}
