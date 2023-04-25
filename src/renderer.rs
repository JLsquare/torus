use crate::ray::Ray;
use crate::raycaster::Raycaster;
use crate::vector::Vector3;
use pixels::Pixels;

fn get_pixel_color(raycaster: &Raycaster, ray: &Ray) -> Vector3 {
    if let Some(intersected_voxel) = raycaster.fast_cast_ray_dda(ray, 64) {
        intersected_voxel.color
    } else {
        Vector3::zero()
    }
}

pub fn draw_frame(
    pixels: &mut Pixels,
    raycaster: &Raycaster,
    ray_origin: Vector3,
    window_size: winit::dpi::PhysicalSize<u32>,
    rotation_angle: Vector3,
) {
    let frame = pixels.frame_mut();
    let (width, height) = (window_size.width, window_size.height);
    let fov: f32 = 60.0_f32.to_radians();
    let aspect_ratio = width as f32 / height as f32;

    for y in 0..height {
        for x in 0..width {
            let ndc_x = (x as f32 + 0.5) / width as f32 * 2.0 - 1.0;
            let ndc_y = -((y as f32 + 0.5) / height as f32 * 2.0 - 1.0);

            let ray_direction = Vector3::new(
                ndc_x * aspect_ratio * (fov / 2.0).tan(),
                ndc_y * (fov / 2.0).tan(),
                1.0,
            );

            let rotated_ray_direction = ray_direction.rotate(rotation_angle);

            let ray = Ray::new(ray_origin, rotated_ray_direction.normalize());

            let color = get_pixel_color(raycaster, &ray);
            let index = (x + y * width) as usize;

            frame[index * 4] = (color.x * 255.0) as u8;
            frame[index * 4 + 1] = (color.y * 255.0) as u8;
            frame[index * 4 + 2] = (color.z * 255.0) as u8;
            frame[index * 4 + 3] = 255;
        }
    }
}
