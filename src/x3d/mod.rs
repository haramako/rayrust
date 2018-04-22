mod vec3;
mod mat4;
mod ray;
mod scene;
mod rect;
mod sphere;
mod color;

pub type Vec3 = vec3::Vec3;
pub type Mat4 = mat4::Mat4;
pub type Ray = ray::Ray;
pub type RayCaster = scene::RayCaster;
pub type Sphere = sphere::Sphere;
pub type Rect = rect::Rect;
pub type RayHit<'a> = scene::RayHit<'a>;
pub type Scene = scene::Scene;
pub type Material = scene::Material;
pub type Entity = scene::Entity;
pub type Color = color::Color;
