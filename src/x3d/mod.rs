extern crate cgmath;

mod color;
mod material;
mod ray;
mod rect;
mod scene;
mod sphere;

pub use x3d::cgmath::prelude::*;
pub use x3d::cgmath::*;

pub type Point = cgmath::Point3<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;
pub type Ray = ray::Ray;
pub type Shape = scene::Shape;
pub type Sphere = sphere::Sphere;
pub type Rect = rect::Rect;
pub type RayHit<'a> = scene::RayHit<'a>;
pub type Scene = scene::Scene;
pub type Material = material::Material;
pub type Entity = scene::Entity;
pub type Color = color::Color;
