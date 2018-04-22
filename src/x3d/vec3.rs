use std::ops;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub data: [f32; 3],
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        &mut self.data[index]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] + _rhs[0], self[1] + _rhs[1], self[2] + _rhs[2]],
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            data: [self[0] - _rhs[0], self[1] - _rhs[1], self[2] - _rhs[2]],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            data: [0.0, 0.0, 0.0],
        }
    }

    pub fn xy(x: f32, y: f32) -> Vec3 {
        Vec3 { data: [x, y, 0.0] }
    }

    pub fn xyz(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn magnitude_square(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_square().sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let m = 1.0 / self.magnitude();
        Vec3::xyz(self[0] * m, self[1] * m, self[2] * m)
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn product(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            data: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }
}
