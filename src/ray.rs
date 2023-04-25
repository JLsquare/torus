use crate::vector::Vector3;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        let normalized_direction = direction.normalize();
        Self {
            origin,
            direction: normalized_direction,
        }
    }

    pub fn point_at_distance(&self, distance: f32) -> Vector3 {
        self.origin.add(&self.direction.multiply(distance))
    }
}
