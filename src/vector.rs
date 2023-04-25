#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3Int {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vector2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Vector2) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }

    pub fn dot(&self, other: &Vector2) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        self.divide(length)
    }

    pub fn to_vector2int(&self) -> Vector2Int {
        Vector2Int::new(self.x.round() as i32, self.y.round() as i32)
    }

    pub fn to_vector3(&self, z: f32) -> Vector3 {
        Vector3::new(self.x, self.y, z)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Vector3) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vector3) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    pub fn dot(&self, other: &Vector3) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        self.divide(length)
    }

    pub fn to_vector3int(&self) -> Vector3Int {
        Vector3Int::new(
            self.x.round() as i32,
            self.y.round() as i32,
            self.z.round() as i32,
        )
    }

    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn component_div(&self, other: &Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn argmin(&self) -> Option<usize> {
        let (index, _) = [self.x, self.y, self.z]
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;
        Some(index)
    }

    pub fn add_scalar(&self, scalar: f32) -> Self {
        Self::new(self.x + scalar, self.y + scalar, self.z + scalar)
    }

    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub fn add_masked_scalar(&self, scalar: f32, mask: &Vector3) -> Self {
        Self::new(
            self.x + scalar * (mask.x > 0.0) as i32 as f32,
            self.y + scalar * (mask.y > 0.0) as i32 as f32,
            self.z + scalar * (mask.z > 0.0) as i32 as f32,
        )
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        Self {
            x: self.x,
            y: self.y * cos_angle - self.z * sin_angle,
            z: self.y * sin_angle + self.z * cos_angle,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        Self {
            x: self.x * cos_angle + self.z * sin_angle,
            y: self.y,
            z: -self.x * sin_angle + self.z * cos_angle,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        Self {
            x: self.x * cos_angle - self.y * sin_angle,
            y: self.x * sin_angle + self.y * cos_angle,
            z: self.z,
        }
    }

    pub fn rotate(&self, angles: Vector3) -> Self {
        let pitch = angles.x;
        let yaw = angles.y;

        self.rotate_x(pitch).rotate_y(yaw)
    }

    pub fn min_component(&self) -> f32 {
        self.x.min(self.y.min(self.z))
    }

    pub fn add_assign(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Vector2Int {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vector2Int) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn subtract(&self, other: &Vector2Int) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self::new(
            (self.x as f32 * scalar).round() as i32,
            (self.y as f32 * scalar).round() as i32,
        )
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self::new(
            (self.x as f32 / scalar).round() as i32,
            (self.y as f32 / scalar).round() as i32,
        )
    }

    pub fn dot(&self, other: &Vector2Int) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        self.divide(length)
    }

    pub fn to_vector2(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    pub fn to_vector3int(&self, z: i32) -> Vector3Int {
        Vector3Int::new(self.x, self.y, z)
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl Vector3Int {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: &Vector3Int) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vector3Int) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn multiply(&self, scalar: f32) -> Self {
        Self::new(
            (self.x as f32 * scalar).round() as i32,
            (self.y as f32 * scalar).round() as i32,
            (self.z as f32 * scalar).round() as i32,
        )
    }

    pub fn divide(&self, scalar: f32) -> Self {
        Self::new(
            (self.x as f32 / scalar).round() as i32,
            (self.y as f32 / scalar).round() as i32,
            (self.z as f32 / scalar).round() as i32,
        )
    }

    pub fn dot(&self, other: &Vector3Int) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3Int) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        self.divide(length)
    }

    pub fn to_vector3(&self) -> Vector3 {
        Vector3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    pub fn to_vector2int(&self) -> Vector2Int {
        Vector2Int::new(self.x, self.y)
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }
}
