use crate::map::Map;
use crate::ray::Ray;
use crate::voxel::Voxel;

pub struct Raycaster {
    map: Map,
}

impl Raycaster {
    pub fn new(map: Map) -> Self {
        Self { map }
    }

    pub fn cast_ray_old(&self, ray: &Ray, max_step: i32, step_size: f32) -> Option<&Voxel> {
        let mut current_step = 0;

        while current_step < max_step {
            let current_distance = current_step as f32 * step_size;
            let current_point = ray.point_at_distance(current_distance);
            let grid_x = current_point.x.floor() as i32;
            let grid_y = current_point.y.floor() as i32;
            let grid_z = current_point.z.floor() as i32;

            if !self.map.is_within_bounds(grid_x, grid_y, grid_z) {
                break;
            }

            if let Some(voxel) = self.map.get_voxel(grid_x, grid_y, grid_z) {
                if !voxel.is_empty {
                    return Some(voxel);
                }
            }

            current_step += 1;
        }

        None
    }

    pub fn cast_ray_dda(&self, ray: &Ray, max_step: i32) -> Option<&Voxel> {
        let mut grid_pos = ray.origin.floor();
        let grid_step = ray.direction.signum();
        let mut t_max = (grid_pos
            .add_masked_scalar(1.0, &grid_step)
            .subtract(&ray.origin))
        .component_div(&ray.direction)
        .abs();
        let t_delta = grid_step.component_div(&ray.direction).abs();

        for _ in 0..max_step {
            if !self
                .map
                .is_within_bounds(grid_pos.x as i32, grid_pos.y as i32, grid_pos.z as i32)
            {
                break;
            }

            if let Some(voxel) =
                self.map
                    .get_voxel(grid_pos.x as i32, grid_pos.y as i32, grid_pos.z as i32)
            {
                if !voxel.is_empty {
                    return Some(voxel);
                }
            }

            let step_axis = t_max.argmin().unwrap_or(0);
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
        }

        None
    }

    pub fn fast_cast_ray_dda(&self, ray: &Ray, max_step: i32) -> Option<&Voxel> {
        let mut grid_pos = ray.origin.floor();
        let grid_step = ray.direction.signum();
        let mut t_max = (grid_pos
            .add_masked_scalar(1.0, &grid_step)
            .subtract(&ray.origin))
        .component_div(&ray.direction)
        .abs();
        let t_delta = grid_step.component_div(&ray.direction).abs();

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
                let step = (distance / ray.direction.magnitude()).ceil() as i32;
                for _ in 0..step {
                    let step_axis = t_max.argmin().unwrap_or(0);
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
                        return Some(voxel);
                    }
                }

                let step_axis = t_max.argmin().unwrap_or(0);
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

        None
    }
}
