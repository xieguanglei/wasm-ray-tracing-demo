use std::ops;


#[derive(Debug, Clone, Copy)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec {
    pub fn zero() -> Self {
        Vec {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec { x: x, y: y, z: z }
    }

    pub fn norm(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec::new(self.x / len, self.y / len, self.z / len)
    }
}

impl ops::Add<Vec> for Vec {
    type Output = Vec;

    fn add(self, rhs: Vec) -> Self::Output {
        Vec::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vec> for Vec {
    type Output = Vec;

    fn sub(self, rhs: Vec) -> Self::Output {
        Vec::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec> for Vec {
    type Output = Vec;

    fn mul(self, rhs: Vec) -> Self::Output {
        Vec::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for Vec {
    type Output = Vec;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::BitAnd<Vec> for Vec {
    type Output = f64;

    fn bitand(self, rhs: Vec) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Rem<Vec> for Vec {
    type Output = Vec;

    fn rem(self, rhs: Vec) -> Self::Output {
        Vec::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

#[cfg(test)]
mod tests;
