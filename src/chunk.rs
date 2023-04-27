use crate::map::Map;
use crate::perlin::PerlinGenerator;
use crate::voxel::Voxel;
use nalgebra::Vector3;

pub type ChunkPosition = (i32, i32, i32);

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    data: Vec<Voxel>,
    distance_map: Vec<u8>,
    pub position: ChunkPosition,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        let data = vec![Voxel::empty(); 4096];
        let distance_map = vec![0; 4096];
        Self {
            data,
            distance_map,
            position,
        }
    }

    pub fn get_voxel(&self, x: u8, y: u8, z: u8) -> Option<&Voxel> {
        if x < 16 && y < 16 && z < 16 {
            Some(&self.data[Chunk::get_index(x, y, z)])
        } else {
            None
        }
    }

    pub fn get_voxel_mut(&mut self, x: u8, y: u8, z: u8) -> Option<&mut Voxel> {
        if x < 16 && y < 16 && z < 16 {
            Some(&mut self.data[Chunk::get_index(x, y, z)])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: u8, y: u8, z: u8, voxel: Voxel) -> () {
        if x < 16 && y < 16 && z < 16 {
            self.data[Chunk::get_index(x, y, z)] = voxel;
        }
    }

    pub fn generate(&mut self, pos: Vector3<i32>, perlin: &PerlinGenerator) {
        let scale = 16.0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let noise = perlin.get(
                        (x + pos.x) as f64 / scale,
                        (y + pos.y) as f64 / scale,
                        (z + pos.z) as f64 / scale,
                    );
                    if noise > 0.5 {
                        let color = (noise * 255.0) as u8;
                        self.set(
                            x as u8,
                            y as u8,
                            z as u8,
                            Voxel::new(Vector3::new(color, color, color)),
                        );
                    }
                }
            }
        }
    }

    pub fn set_distance(&mut self, x: u8, y: u8, z: u8, distance: u8) {
        if x < 16 && y < 16 && z < 16 {
            self.distance_map[Chunk::get_index(x, y, z)] = distance;
        }
    }

    pub fn get_distance(&self, x: u8, y: u8, z: u8) -> u8 {
        if x < 16 && y < 16 && z < 16 {
            self.distance_map[Chunk::get_index(x, y, z)]
        } else {
            1
        }
    }

    fn get_index(x: u8, y: u8, z: u8) -> usize {
        z as usize * 256 + y as usize * 16 + x as usize
    }

    pub fn generate_distance_map(&mut self, map: &Map, radius: i32) {
        let start_distance = radius * radius * radius;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let mut min_distance = start_distance;
                    for dx in -radius..=radius {
                        for dy in -radius..=radius {
                            for dz in -radius..=radius {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }

                                if let Some(voxel) = map.get_voxel(
                                    (x + dx + (self.position.0 * 16)) as i32,
                                    (y + dy + (self.position.1 * 16)) as i32,
                                    (z + dz + (self.position.2 * 16)) as i32,
                                ) {
                                    if !voxel.is_empty {
                                        let distance = dx * dx + dy * dy + dz * dz;
                                        min_distance = min_distance.min(distance);
                                    }
                                }
                            }
                        }
                    }
                    self.set_distance(
                        x as u8,
                        y as u8,
                        z as u8,
                        (min_distance as f32 - 1.0).sqrt() as u8,
                    );
                }
            }
        }
    }
}
