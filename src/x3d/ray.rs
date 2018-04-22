use Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
        }
    }

    pub fn new_origin_to(origin: Vec3, to: Vec3) -> Ray {
        let dir = to - origin;
        Ray::new(origin, dir)
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
}
