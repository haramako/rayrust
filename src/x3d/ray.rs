use Point;
use Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray {
            origin: origin,
            dir: dir,
        }
    }

    pub fn new_origin_to(origin: Point, to: Point) -> Ray {
        let dir = to - origin;
        Ray::new(origin, dir)
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}
