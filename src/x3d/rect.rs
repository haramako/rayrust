use x3d::scene::*;
use Entity;
use Point;
use Ray;
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
        // 裏面
        if ray.origin.z > 0.0 || ray.dir.z < 0.0 {
            return None;
        }

        let t = -ray.origin.z / ray.dir.z;
        if t < 0.0 {
            return None;
        }

        let at = ray.at(t);
        let u = at.x;
        let v = at.y;
        if u < -0.5 || u > 0.5 || v < -0.5 || v >= 0.5 {
            return None;
        }

        Some(RayHit {
            entity: entity,
            t: t,
        })
    }

    fn normal(&self, _position: Point) -> Vec3 {
        Vec3::new(0.0, 0.0, -1.0)
    }
}
