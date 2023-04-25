use std::collections::HashMap;
use crate::chunk::{Chunk, ChunkPosition};
use crate::voxel::Voxel;
use crate::perlin::PerlinGenerator;
use crate::vector::{Vector3, Vector3Int};

#[derive(Debug, Default, Clone)]
pub struct Map {
    pub chunks: HashMap<ChunkPosition, Chunk>
}

impl Map {
    pub fn new() -> Self {
        Self { chunks: HashMap::new() }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<&Chunk> {
        let chunk_x = x.div_euclid(16);
        let chunk_y = y.div_euclid(16);
        let chunk_z = z.div_euclid(16);
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
            chunk.get(voxel_x as isize, voxel_y as isize, voxel_z as isize)
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
            chunk.get_mut(voxel_x as isize, voxel_y as isize, voxel_z as isize)
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
            chunk.get_distance(voxel_x as usize, voxel_y as usize, voxel_z as usize)
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
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let mut chunk = Chunk::new();
                    chunk.generate(Vector3Int::new(x, y, z).multiply(16.0).to_vector3(), perlin);
                    self.set(x * 16, y * 16, z * 16, chunk);
                }
            }
        }
    }

    pub fn generate_distance_maps(&mut self, radius: u8) {
        for chunk in self.chunks.values_mut() {
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let mut min_distance = radius as isize;
                        for dx in -(radius as isize)..=(radius as isize) {
                            for dy in -(radius as isize)..=(radius as isize) {
                                for dz in -(radius as isize)..=(radius as isize) {
                                    if dx == 0 && dy == 0 && dz == 0 {
                                        continue;
                                    }

                                    if let Some(voxel) = chunk.get(x as isize + dx, y as isize + dy, z as isize + dz) {
                                        if !voxel.is_empty {
                                            let distance = dx.abs().max(dy.abs()).max(dz.abs());
                                            min_distance = min_distance.min(distance);
                                        }
                                    }
                                }
                            }
                        }
                        chunk.set_distance(x, y, z, min_distance as u8);
                    }
                }
            }
        }
    }
}