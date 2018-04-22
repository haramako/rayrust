use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub data: [f32; 4],
}

impl Color {
    pub fn new() -> Color {
        Color { data: [0.0; 4] }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { data: [r, g, b, a] }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            data: [r, g, b, 1.0],
        }
    }

    pub fn white() -> Color {
        Color::from_rgb(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::from_rgb(0.0, 0.0, 0.0)
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            data: [
                self.data[0] * rhs,
                self.data[1] * rhs,
                self.data[2] * rhs,
                self.data[3] * rhs,
            ],
        }
    }
}
