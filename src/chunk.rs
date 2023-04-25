
use crate::perlin::PerlinGenerator;
use crate::vector::Vector3;
use crate::voxel::Voxel;

pub type ChunkPosition = (i32, i32, i32);

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    data: Vec<Voxel>,
    distance_map: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        let data = vec![Voxel::empty(); 4096];
        let distance_map = vec![0; 4096];
        Self { data, distance_map }
    }

    pub fn get(&self, x: isize, y: isize, z: isize) -> Option<&Voxel> {
        if x >= 0 && y >= 0 && z >= 0 && x < 16 && y < 16 && z < 16 {
            let x = x as usize;
            let y = y as usize;
            let z = z as usize;
            Some(&self.data[z * 256 + y * 16 + x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: isize, y: isize, z: isize) -> Option<&mut Voxel> {
        if x >= 0 && y >= 0 && z >= 0 && x < 16 && y < 16 && z < 16 {
            let x = x as usize;
            let y = y as usize;
            let z = z as usize;
            Some(&mut self.data[z * 256 + y * 16 + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, voxel: Voxel) -> Result<(), &'static str> {
        if x < 16 && y < 16 && z < 16 {
            self.data[z * 256 + y * 16 + x] = voxel;
            Ok(())
        } else {
            Err("Invalid grid position")
        }
    }

    pub fn generate(&mut self, pos: Vector3, perlin: &PerlinGenerator) {
        let scale = 1.0 / 16.0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let noise = perlin.get(
                        (x as f64 + pos.x as f64) * scale,
                        (y as f64 + pos.y as f64) * scale,
                        (z as f64 + pos.z as f64) * scale,
                    );
                    if noise > 0.5 {
                        self.set(
                            x as usize,
                            y as usize,
                            z as usize,
                            Voxel::new(Vector3::new(noise as f32, noise as f32, noise as f32)),
                        )
                        .unwrap();
                    }
                }
            }
        }
    }

    pub fn set_distance(&mut self, x: usize, y: usize, z: usize, distance: u8) {
        self.distance_map[z * 256 + y * 16 + x] = distance;
    }

    pub fn get_distance(&self, x: usize, y: usize, z: usize) -> u8 {
        self.distance_map[z * 256 + y * 16 + x]
    }
}
