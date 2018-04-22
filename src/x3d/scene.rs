use Vec3;
use Ray;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub data: [f32; 4],
}

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub albedo: f32,
    pub reflect: f32,
    pub emission: Color,
}

pub struct Scene {
    pub objects: Vec<Box<Entity>>,
}

#[derive(Debug)]
pub struct Sphere {
    pub radius: f32,
}

pub struct Camera {
    pub position: Vec3,
    pub fov: f32,
}

#[derive(Debug)]
pub struct RayHit<'a> {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub entity: &'a Entity,
}

#[derive(Debug)]
pub struct Entity {
    pub position: Vec3,
    pub material: Material,
    pub shape: Box<RayCaster>,
}

impl Entity {
    pub fn new(position: Vec3, shape: Box<RayCaster>) -> Entity {
        Entity {
            position: position,
            material: Material::new(),
            shape: shape,
        }
    }
}

pub trait RayCaster: fmt::Debug {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>>;
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
            albedo: 1.0,
            reflect: 0.0,
            emission: Color::from_rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Color {
    pub fn new() -> Color {
        Color { data: [0.0; 4] }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { data: [r, g, b, a] }
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

impl RayCaster for Sphere {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let oc = ray.origin - entity.position;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(oc);
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let d = b * b - 4.0 * a * c;
        if d <= 0.0 {
            None
        } else {
            let t = (-b - d.sqrt()) / (2.0 * a);
            let p = ray.at(t);
            let n = (p - entity.position).normalized();
            Some(RayHit {
                entity: entity,
                t: t,
                position: p,
                normal: n,
            })
        }
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius: radius }
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
}
