use x3d::scene::*;
use Ray;
use Entity;

#[derive(Debug, Clone)]
pub struct Rect {}

impl Rect {
    pub fn new() -> Rect {
        Rect {}
    }
}

impl RayCaster for Rect {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let inv = entity.matrix.invert();
        let transformed_origin = inv * ray.origin;
        let transformed_dir = inv.without_translate() * ray.dir;
        let t = -transformed_origin.z() / transformed_dir.z();
        if t > 0.0 {
            Some(RayHit {
                entity: entity,
                t: t,
                normal: transformed_origin,
                position: transformed_origin + transformed_dir * t,
            })
        } else {
            None
        }
    }
}
