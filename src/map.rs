use crate::chunk::{Chunk, ChunkPosition};
use crate::perlin::PerlinGenerator;
use crate::voxel::Voxel;
use nalgebra::Vector3;
use rayon::scope;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
        for x in -4..=4 {
            for y in -4..=4 {
                for z in -4..=4 {
                    let mut chunk = Chunk::new((x, y, z));
                    chunk.generate(Vector3::new(x, y, z) * 16, perlin);
                    self.set(x * 16, y * 16, z * 16, chunk);
                }
            }
        }
    }

    pub fn generate_all_distance_maps(&mut self, radius: i32) {
        let map_clone = Arc::new(self.clone());
        let total_chunks = self.chunks.len();
        let time = std::time::Instant::now();
        let completed_chunks = Arc::new(Mutex::new(0));

        scope(|s| {
            for chunk in self.chunks.values_mut() {
                let map_clone = Arc::clone(&map_clone);
                let completed_chunks = Arc::clone(&completed_chunks);

                s.spawn(move |_| {
                    chunk.generate_distance_map(&map_clone, radius);
                    let mut completed_chunks = completed_chunks.lock().unwrap();
                    *completed_chunks += 1;

                    if *completed_chunks % 64 == 0 {
                        let progress = *completed_chunks as f32 / total_chunks as f32 * 100.0;
                        let elapsed = time.elapsed();
                        let cps = *completed_chunks as f32 / elapsed.as_secs_f32();

                        println!(
                            "Generating distance map for chunk ({}, {}, {}), {:.2}% | CPS: {:.2}",
                            chunk.position.0, chunk.position.1, chunk.position.2, progress, cps
                        );
                    }
                });
            }
        });

        let total_elapsed = time.elapsed().as_secs_f32();
        let total_completed = *completed_chunks.lock().unwrap();
        println!(
            "Distance maps generated! Time: {:.2} seconds | CPS: {:.2}",
            total_elapsed,
            total_completed as f32 / total_elapsed
        );
    }
}
