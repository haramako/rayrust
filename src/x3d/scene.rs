use std::fmt;
use Vec3;
use Ray;
use Mat4;
use Material;

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
    pub entity: &'a Entity,
}

#[derive(Debug)]
pub struct Entity {
    pub matrix: Mat4,
    pub nt_matrix: Mat4,
    pub inv_matrix: Mat4,
    pub inv_nt_matrix: Mat4,
    pub material: Material,
    pub shape: Box<Shape>,
}

impl Entity {
    pub fn new(matrix: Mat4, shape: Box<Shape>) -> Entity {
        Entity {
            matrix: matrix,
            nt_matrix: matrix.without_translate(),
            inv_matrix: matrix.invert(),
            inv_nt_matrix: matrix.invert().without_translate(),
            material: Material::new(),
            shape: shape,
        }
    }
}

pub trait Shape: fmt::Debug {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>>;
    fn normal(&self, position: Vec3) -> Vec3;
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
}
