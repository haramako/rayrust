#![allow(dead_code)]
extern crate cgmath;
extern crate getopts;
extern crate image;
extern crate rand;
extern crate rayon;

mod x3d;

use getopts::Options;
use image::{ImageBuffer, Rgba};
use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;
use std::{env, f32};
use x3d::*;

#[derive(Copy, Clone)]
struct RenderParam {
    pub min_rad: f32,
}

struct State {
    pub rng: Box<rand::Rng>,
}

fn main() {
    /*
    let ax = Vec3::xyz(1.0, 0.0, 0.0);
    let ay = Vec3::xyz(0.0, 1.0, 0.0);
    let v1 = Vec3::xyz(1.0, 2.0, 3.0);
    let v2 = Vec3::xyz(1.0, 2.0, 3.0);
    println!("add/sub {:?}", v1 + v2 - v1 * 2.0);
    println!("dot {:?}", v1.dot(v2));
    println!("product {:?}", ax.product(ay));

    println!("{:?}", Vec3::xy(1.0, 2.0));
    println!("{:?}", Vec3::xyz(1.0, 2.0, 3.0));
    println!("{:?}", Mat4::scale(2.0));
    println!("{:?}", Mat4::identity());
    */

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("s", "size", "Picture size", "PIXEL");
    opts.optopt("q", "quarity", "quarity(default=2)", "QUARITY");
    let matches = opts.parse(&args[1..]).expect("invalid argument");

    let read_opt = |m: &getopts::Matches, name, default| -> i32 {
        m.opt_str(name)
            .map(|v| v.parse::<i32>().expect("invalid size"))
            .unwrap_or(default)
    };
    let size = read_opt(&matches, "s", 32);
    let quarity = read_opt(&matches, "q", 5);

    let param = RenderParam {
        min_rad: (0.3f32).powf(quarity as f32),
    };

    let w: i32 = size;
    let h: i32 = size;

    let mut img = ImageBuffer::from_fn(w as u32, h as u32, |_, _| {
        image::Rgba([0u8, 0u8, 0u8, 255u8])
    });

    let scene = make_scene();

    let bsize = 32;
    let xx = (0..h / bsize)
        .flat_map(move |y| (0..w / bsize).map(move |x| (y, x)))
        .collect::<Vec<(i32, i32)>>();

    let xx = xx.par_iter()
        .map(|(bx, by)| {
            let buf = render_block(
                &param,
                &scene,
                bx * bsize,
                by * bsize,
                bsize as usize,
                bsize as usize,
                w as usize,
                h as usize,
            );
            println!("render {:?}", (bx, by));
            ((bx, by, buf))
        })
        .collect::<Vec<_>>();

    for (bx, by, buf) in xx.iter() {
        for (dy, row) in buf.iter().enumerate() {
            for (dx, color) in row.iter().enumerate() {
                let ix = (*bx * bsize) as u32 + dx as u32;
                let iy = (*by * bsize) as u32 + dy as u32;
                img[(ix, iy)] = color_to_rgba(color);
            }
        }
    }

    img.save(Path::new("test.png")).unwrap();
}

fn render_block(
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
        rng: Box::new(rand::StdRng::new().expect("StdRng::new() error.")),
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

fn make_scene() -> Scene {
    let mut scene = Scene::new();

    let white = Color::from_rgb(1.0, 1.0, 1.0);

    // 球1
    let mut s1 = Entity::new(
        trs(vec3(0.0, 0.2, 0.0), [0.0, 0.0, 0.0], 1.0),
        Box::new(Sphere::new(0.5)),
    );
    s1.material.albedo = white;
    //s1.material.emission = white * 4.0;
    //s1.material.reflect = 0.8;
    scene.objects.push(Box::new(s1));

    // 球2
    let mut s2 = Entity::new(
        Mat4::from_translation(vec3(0.2, 0.3, 0.5)),
        Box::new(Sphere::new(0.5)),
    );
    s2.material.albedo = white;
    //scene.objects.push(Box::new(s2));

    // ライト
    let mut light = Entity::new(
        trs(vec3(0.0, -0.799 * 1.0, 0.0), [-90.0, 0.0, 0.0], 0.7),
        Box::new(Rect::new()),
    );
    light.material.emission = white * 8.0;
    scene.objects.push(Box::new(light));

    // 天井
    let mut roof = Entity::new(
        trs(vec3(0.0, -0.801 * 1.0, 0.0), [90.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    roof.material.albedo = white;
    scene.objects.push(Box::new(roof));

    // 床
    let mut floor = Entity::new(
        trs(vec3(0.0, 0.801, 0.0), [-90.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    floor.material.albedo = white;
    scene.objects.push(Box::new(floor));

    // 後ろの壁
    let mut w_back = Entity::new(
        trs(vec3(0.0, 0.0, 0.803), [0.0, 0.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_back.material.albedo = white;
    scene.objects.push(Box::new(w_back));

    // 手前の壁
    let mut w_front = Entity::new(
        trs(vec3(0.0, 0.0, -0.803), [0.0, 180.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_front.material.albedo = white;
    scene.objects.push(Box::new(w_front));

    // 右の壁
    let mut w_right = Entity::new(
        trs(vec3(0.8, 0.0, 0.0), [0.0, 90.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_right.material.albedo = Color::from_rgb(2.0, 0.2, 0.1);
    w_right.material.emission = Color::from_rgb(0.5, 0.0, 0.0);
    scene.objects.push(Box::new(w_right));

    // 左の壁
    let mut w_left = Entity::new(
        trs(vec3(-0.8, 0.0, 0.0), [0.0, -90.0, 0.0], 3.0),
        Box::new(Rect::new()),
    );
    w_left.material.albedo = Color::from_rgb(0.2, 0.8, 0.1);
    scene.objects.push(Box::new(w_left));

    scene
}

fn trs(translate: Vec3, rotate: [f32; 3], scale: f32) -> Mat4 {
    Mat4::from_translation(translate) * Mat4::from_scale(scale) * Mat4::from_angle_x(Deg(rotate[0]))
        * Mat4::from_angle_y(Deg(rotate[1])) * Mat4::from_angle_z(Deg(rotate[2]))
}

fn without_translate(m: &Mat4) -> Mat4 {
    let mut r = *m;
    r.w[0] = 0.0;
    r.w[1] = 0.0;
    r.w[2] = 0.0;
    r
}

/// 特定のVectorからランダムな反射方向を取得する
fn random_vector(rng: &mut Box<rand::Rng>, v: Vec3) -> Vec3 {
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

fn render(
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

fn vec_to_color(v: &Vec3) -> Color {
    Color {
        data: [
            (v[0] + 1.0) / 2.0,
            (v[1] + 1.0) / 2.0,
            (v[2] + 1.0) / 2.0,
            1.0,
        ],
    }
}

fn color_to_rgba(c: &Color) -> Rgba<u8> {
    let mut r = [0u8; 4];
    r[0] = (to_srgb(c.data[0]) * 255.0).min(255.0).max(0.0) as u8;
    r[1] = (to_srgb(c.data[1]) * 255.0).min(255.0).max(0.0) as u8;
    r[2] = (to_srgb(c.data[2]) * 255.0).min(255.0).max(0.0) as u8;
    r[3] = 255;
    Rgba(r)
}

fn to_srgb(v: f32) -> f32 {
    v.powf(1.0 / 2.2)
}
