use crate::chunk::{Chunk, ChunkPosition};
use crate::perlin::PerlinGenerator;
use crate::voxel::Voxel;
use nalgebra::Vector3;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Map {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<&Chunk> {
        let chunk_x = x.div_euclid(16) as i32;
        let chunk_y = y.div_euclid(16) as i32;
        let chunk_z = z.div_euclid(16) as i32;
        self.chunks.get(&(chunk_x, chunk_y, chunk_z))
    }

    pub fn get_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Chunk> {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        self.chunks.get_mut(&(chunk_x, chunk_y, chunk_z))
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, chunk: Chunk) {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        self.chunks.insert((chunk_x, chunk_y, chunk_z), chunk);
    }

    pub fn get_voxel(&self, x: i32, y: i32, z: i32) -> Option<&Voxel> {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let chunk = self.chunks.get(&(chunk_x, chunk_y, chunk_z));
        if let Some(chunk) = chunk {
            let voxel_x = x.rem_euclid(16);
            let voxel_y = y.rem_euclid(16);
            let voxel_z = z.rem_euclid(16);
            chunk.get_voxel(voxel_x as u8, voxel_y as u8, voxel_z as u8)
        } else {
            None
        }
    }

    pub fn get_voxel_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Voxel> {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let chunk = self.chunks.get_mut(&(chunk_x, chunk_y, chunk_z));
        if let Some(chunk) = chunk {
            let voxel_x = x.rem_euclid(16);
            let voxel_y = y.rem_euclid(16);
            let voxel_z = z.rem_euclid(16);
            chunk.get_voxel_mut(voxel_x as u8, voxel_y as u8, voxel_z as u8)
        } else {
            None
        }
    }

    pub fn get_distance(&self, x: i32, y: i32, z: i32) -> u8 {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        let chunk = self.chunks.get(&(chunk_x, chunk_y, chunk_z));
        if let Some(chunk) = chunk {
            let voxel_x = x.rem_euclid(16);
            let voxel_y = y.rem_euclid(16);
            let voxel_z = z.rem_euclid(16);
            chunk.get_distance(voxel_x as u8, voxel_y as u8, voxel_z as u8)
        } else {
            0
        }
    }

    pub fn is_within_bounds(&self, x: i32, y: i32, z: i32) -> bool {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
        self.chunks.contains_key(&(chunk_x, chunk_y, chunk_z))
    }

    pub fn generate(&mut self, perlin: &PerlinGenerator) {
        for x in -8..=8 {
            for y in -8..=8 {
                for z in 0..=8 {
                    let mut chunk = Chunk::new((x, y, z));
                    chunk.generate(Vector3::new(x, y, z) * 16, perlin);
                    self.set(x * 16, y * 16, z * 16, chunk);
                }
            }
        }
    }
}
