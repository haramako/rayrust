use Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub albedo: Color,
    pub reflect: f32,
    pub emission: Color,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::from_rgb(1.0, 0.0, 0.0),
            albedo: Color::from_rgb(0.0, 0.0, 0.0),
            reflect: 0.0,
            emission: Color::from_rgb(0.0, 0.0, 0.0),
        }
    }
}
