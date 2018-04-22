use std::fmt;
use Vec3;
use Ray;
use Mat4;
use Color;

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
    pub matrix: Mat4,
    pub material: Material,
    pub shape: Box<RayCaster>,
}

impl Entity {
    pub fn new(matrix: Mat4, shape: Box<RayCaster>) -> Entity {
        Entity {
            matrix: matrix,
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

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
}
