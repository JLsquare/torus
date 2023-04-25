use crate::vector::Vector3;

#[derive(Debug, Default, Clone)]
pub struct Voxel {
    pub color: Vector3,
    pub is_empty: bool,
}

impl Voxel {
    pub fn new(color: Vector3) -> Self {
        Self {
            color,
            is_empty: false,
        }
    }

    pub fn empty() -> Self {
        Self {
            color: Vector3::new(0.0, 0.0, 0.0),
            is_empty: true,
        }
    }
}
