#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() - self.z * angle.sin(),
            y: self.y,
            z: self.x * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }
}
