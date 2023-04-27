use crate::renderer::Renderer;
use nalgebra::{Rotation3, Vector3};
use pixels::Pixels;

pub struct Camera {
    pub renderer: Renderer,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

impl Camera {
    pub fn new(
        renderer: Renderer,
        position: Vector3<f32>,
        rotation: Vector3<f32>,
        movement_speed: f32,
        rotation_speed: f32,
    ) -> Self {
        Self {
            renderer,
            position,
            rotation,
            movement_speed,
            rotation_speed,
        }
    }

    pub fn draw_frame(&self, pixels: &mut Pixels) {
        self.renderer
            .draw_frame(pixels, self.position, self.rotation);
    }

    pub fn move_forward(&mut self) {
        let rotation_matrix =
            Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let direction = rotation_matrix * Vector3::new(0.0, 0.0, -1.0);
        self.position += direction * self.movement_speed;
    }

    pub fn move_backward(&mut self) {
        let rotation_matrix =
            Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let direction = rotation_matrix * Vector3::new(0.0, 0.0, 1.0);
        self.position += direction * self.movement_speed;
    }

    pub fn move_left(&mut self) {
        let rotation_matrix =
            Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let direction = rotation_matrix * Vector3::new(-1.0, 0.0, 0.0);
        self.position += direction * self.movement_speed;
    }

    pub fn move_right(&mut self) {
        let rotation_matrix =
            Rotation3::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let direction = rotation_matrix * Vector3::new(1.0, 0.0, 0.0);
        self.position += direction * self.movement_speed;
    }

    pub fn move_up(&mut self) {
        self.position += Vector3::new(0.0, self.movement_speed, 0.0);
    }

    pub fn move_down(&mut self) {
        self.position += Vector3::new(0.0, -self.movement_speed, 0.0);
    }

    pub fn rotate_left(&mut self) {
        self.rotation.y -= self.rotation_speed;
    }

    pub fn rotate_right(&mut self) {
        self.rotation.y += self.rotation_speed;
    }

    pub fn rotate_up(&mut self) {
        self.rotation.x -= self.rotation_speed;
    }

    pub fn rotate_down(&mut self) {
        self.rotation.x += self.rotation_speed;
    }
}
