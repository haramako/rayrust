extern crate rand;

use rand::prelude::*;
use std::cmp::Ordering;
use x3d::*;

#[derive(Clone)]
pub struct RenderParam {
    pub min_rad: f32,
}

pub struct State {
    pub rng: Box<rand::rngs::StdRng>,
}

/// 特定のVectorからランダムな反射方向を取得する
fn random_vector(rng: &mut Box<rand::rngs::StdRng>, v: Vec3) -> Vec3 {
    loop {
        let x = rng.gen_range(-1.0, 1.0);
        let y = rng.gen_range(-1.0, 1.0);
        let z = rng.gen_range(0.0, 1.0);
        let rand_vec = vec3(x, y, z);
        if rand_vec.magnitude2() <= 1.0 {
            let rot = Quaternion::from_arc(vec3(0.0, 0.0, 1.0), rand_vec, None);
            return rot.rotate_vector(v);
        }
    }
}

pub fn render_block(
    param: &RenderParam,
    scene: &Scene,
    sx: i32,
    sy: i32,
    w: usize,
    h: usize,
    total_w: usize,
    total_h: usize,
) -> Vec<Vec<Color>> {
    let mut stat = State {
        rng: Box::new(rand::rngs::StdRng::from_rng(thread_rng()).expect("StdRng::new() error.")),
    };

    let mut buf = (0..h).map(|_| vec![Color::new(); w]).collect::<Vec<_>>();
    for _y in 0..(h as i32) {
        for _x in 0..(w as i32) {
            let x = ((sx + _x - (total_w as i32) / 2) as f32) / (total_w as f32);
            let y = ((sy + _y - (total_h as i32) / 2) as f32) / (total_h as f32);
            let ray = Ray::new(Point3::new(0.0, 0.0, -2.0), vec3(x, y, 1.0).normalize());
            let color = render(&param, &mut stat, &ray, 1.0, &scene);
            buf[_y as usize][_x as usize] = color.unwrap_or(Color::from_rgb(1.0, 0.0, 0.0));
        }
    }
    buf
}

pub fn render(
    param: &RenderParam,
    stat: &mut State,
    ray: &Ray,
    rad: f32,
    scene: &Scene,
) -> Option<Color> {
    let hit = rayhit(ray, scene);
    hit.map(|h| {
        let mat = &h.entity.material;
        let shape = &h.entity.shape;
        let local_ray = Ray::new(
            h.entity.inv_matrix.transform_point(ray.origin),
            h.entity.inv_matrix.transform_vector(ray.dir),
        );
        let local_at = local_ray.at(h.t);
        let local_normal = shape.normal(local_at);
        let at = ray.at(h.t);
        let normal = (h.entity.matrix.transform_vector(local_normal)).normalize();

        let mut albedo = Color::new();
        if rad > param.min_rad {
            let div = ((rad / param.min_rad) as i32).min(32).max(2);
            let div_rad = rad / (div as f32);

            for _ in 0..div {
                let rand_vec = random_vector(&mut stat.rng, normal);
                let next_ray = Ray::new(at, rand_vec);
                render(param, stat, &next_ray, div_rad, scene).map(|c| {
                    albedo = albedo + c;
                });
            }
        }

        /*
        if rad * mat.reflect > MIN_RAD {
            let normal = normal;
            //let normal = Vec3::xyz(x,y,z).normalized();
            let ray3 = Ray::new(at, normal);
            let c = render(stat, &ray3, rad * mat.reflect, scene);
            c.map(|cc| col = col + cc);
        }
        */

        albedo * mat.albedo + mat.emission * rad
    })
}

fn rayhit<'a>(ray: &Ray, scene: &'a Scene) -> Option<RayHit<'a>> {
    scene
        .objects
        .iter()
        .flat_map(|&ref obj| {
            let local_ray = Ray::new(
                obj.inv_matrix.transform_point(ray.origin),
                obj.inv_matrix.transform_vector(ray.dir),
            );
            obj.shape.ray_cast(&obj, &local_ray)
        })
        .min_by(|a, b| f32_cmp(a.t, b.t))
}

fn f32_cmp(a: f32, b: f32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
