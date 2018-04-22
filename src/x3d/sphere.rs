use x3d::scene::*;
use Ray;
use Entity;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub radius: f32,
}

impl RayCaster for Sphere {
    fn ray_cast<'a, 'b>(&self, entity: &'b Entity, ray: &'a Ray) -> Option<RayHit<'b>> {
        let inv = entity.matrix.invert();
        let transformed_origin = inv * ray.origin;
        let transformed_dir = inv.without_translate() * ray.dir;
        let oc = transformed_origin;
        let a = transformed_dir.dot(transformed_dir);
        let b = 2.0 * transformed_dir.dot(oc);
        let c = oc.dot(oc) - self.radius.powf(2.0);
        let d = b * b - 4.0 * a * c;
        if d <= 0.0 {
            None
        } else {
            let t = (-b - d.sqrt()) / (2.0 * a);
            let p = ray.at(t);
            let n = p.normalized();
            Some(RayHit {
                entity: entity,
                t: t,
                position: p,
                normal: n,
            })
        }
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Sphere {
        Sphere { radius: radius }
    }
}
