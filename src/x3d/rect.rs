use x3d::scene::*;
use Ray;
use Entity;
use Vec3;

#[derive(Debug, Clone)]
pub struct Rect {}

impl Rect {
    pub fn new() -> Rect {
        Rect {}
    }
}

impl Shape for Rect {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let t = -ray.origin.z() / ray.dir.z();
        let at = ray.at(t);
        let u = at.x();
        let v = at.y();
        if t > 0.0 && u >= -0.5 && u < 0.5 && v >= -0.5 && v < 0.5 {
            Some(RayHit {
                entity: entity,
                t: t,
            })
        } else {
            None
        }
    }

    fn normal(&self, _position: Vec3) -> Vec3 {
        Vec3::xyz(0.0, 0.0, -1.0)
    }
}
