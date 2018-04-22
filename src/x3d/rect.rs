use x3d::scene::*;
use Ray;

#[derive(Debug)]
pub struct Rect {}

impl Rect {
    pub fn new() -> Rect {
        Rect {}
    }
}

impl RayCaster for Rect {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let t = (entity.position.z() - ray.origin.z()) / ray.dir.z();
        Some(RayHit {
            entity: entity,
            t: t,
            normal: entity.position,
            position: ray.origin + ray.dir * t,
        })
    }
}
