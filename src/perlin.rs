use noise::{Perlin, NoiseFn};

pub struct PerlinGenerator {
    pub noise: Perlin,
}

impl PerlinGenerator {
    pub fn new(seed: u32) -> Self {
        let noise = Perlin::new(seed);
        Self { noise }
    }

    pub fn get(&self, x: f64, y: f64, z: f64) -> f64 {
        self.noise.get([x, y, z])
    }
}