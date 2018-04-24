use x3d::scene::*;
use Entity;
use Point;
use Ray;
use Vec3;

use x3d::cgmath::prelude::*;
use x3d::cgmath::InnerSpace;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f32,
}

impl Shape for Sphere {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let oc = ray.origin;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(oc.to_vec());
        let c = oc.dot(oc.to_vec()) - self.radius.powf(2.0);
        let d = b * b - 4.0 * a * c;
        if d <= 0.0 {
            None
        } else {
            let t = (-b - d.sqrt()) / (2.0 * a);
            if t < 0.0 {
                None
            } else {
                Some(RayHit {
                    entity: entity,
                    t: t,
                })
            }
        }
    }

    fn normal(&self, position: Point) -> Vec3 {
        position.to_vec().normalize()
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius: radius }
    }
}
