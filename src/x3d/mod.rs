extern crate cgmath;

mod color;
mod material;
mod ray;
mod rect;
mod renderer;
mod scene;
mod sphere;

pub use self::cgmath::prelude::*;
pub use self::cgmath::*;

pub type Point = cgmath::Point3<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

pub use self::color::Color;
pub use self::material::Material;
pub use self::ray::Ray;
pub use self::rect::Rect;
pub use self::renderer::{render, render_block, RenderParam};
pub use self::scene::{Entity, RayHit, Scene, Shape};
pub use self::sphere::Sphere;
