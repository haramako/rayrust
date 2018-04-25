use std::fmt;
use Mat4;
use Material;
use Point;
use Ray;
use Vec3;

use x3d::cgmath::SquareMatrix;

pub struct Scene {
    pub objects: Vec<Box<Entity>>,
}

pub struct Camera {
    pub position: Point,
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
    pub inv_matrix: Mat4,
    pub material: Material,
    pub shape: Box<Shape>,
}

pub fn without_translate(m: &Mat4) -> Mat4 {
    let mut r = *m;
    r.w[0] = 0.0;
    r.w[1] = 0.0;
    r.w[2] = 0.0;
    r
}

impl Entity {
    pub fn new(matrix: Mat4, shape: Box<Shape>) -> Entity {
        Entity {
            matrix: matrix,
            inv_matrix: matrix.invert().unwrap(),
            material: Material::new(),
            shape: shape,
        }
    }
}

pub trait Shape: fmt::Debug + Sync {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>>;
    fn normal(&self, position: Point) -> Vec3;
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }
}
