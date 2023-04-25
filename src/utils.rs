pub trait IndexMin {
    fn index_min(&self) -> usize;
}

impl IndexMin for (f32, f32, f32) {
    fn index_min(&self) -> usize {
        if self.0 <= self.1 && self.0 <= self.2 {
            0
        } else if self.1 <= self.0 && self.1 <= self.2 {
            1
        } else {
            2
        }
    }
}
