#![allow(dead_code)]
extern crate image;
extern crate rand;

mod x3d;

use image::{ImageBuffer, Rgba};
use std::path::Path;
use std::cmp::Ordering;
use std::f32;
use rand::Rng;

use x3d::*;

struct State {
    pub rng: Box<rand::Rng>,
}

fn main() {
    let mut stat = State {
        rng: Box::new(rand::StdRng::new().expect("StdRng::new() error.")),
    };

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

    let size = 64;
    let w: i32 = size;
    let h: i32 = size;

    let mut img = ImageBuffer::from_fn(w as u32, h as u32, |_, _| {
        image::Rgba([0u8, 0u8, 0u8, 255u8])
    });

    let scene = make_scene();

    for _x in 0..w {
        for _y in 0..h {
            let x = ((_x - w / 2) as f32) / (w as f32);
            let y = ((_y - h / 2) as f32) / (h as f32);
            let ray = Ray::new(Vec3::xyz(0.0, 0.0, -2.0), Vec3::xyz(x, y, 1.0).normalized());
            let color = render(&mut stat, &ray, 1.0, &scene);
            img[(_x as u32, _y as u32)] = color
                .map(|c| color_to_rgba(&c))
                .unwrap_or(Rgba([0, 0, 0, 255]));
        }
        if _x % 10 == 0 {
            println!("{}", _x);
        }
    }

    img.save(Path::new("test.png")).unwrap();
}

fn make_scene() -> Scene {
    let mut scene = Scene::new();

    let white = Color::from_rgb(1.0, 1.0, 1.0);

    // 球1
    let mut s1 = Entity::new(Mat4::translate(-0.3, -0.3, 0.0), Box::new(Sphere::new(0.5)));
    s1.material.albedo = white;
    //s1.material.reflect = 0.8;
    //scene.objects.push(Box::new(s1));

    // 球2
    let mut s2 = Entity::new(Mat4::translate(0.2, 0.3, 0.5), Box::new(Sphere::new(0.5)));
    s2.material.albedo = white;
    scene.objects.push(Box::new(s2));

    // 天井
    let mut light = Entity::new(
        Mat4::scale(0.8) * Mat4::translate(0.0, -0.799, 0.0) * Mat4::rotate_x(90.0),
        Box::new(Rect::new()),
    );
    light.material.emission = white * 4.0;
    scene.objects.push(Box::new(light));

    // 天井
    let mut roof = Entity::new(
        Mat4::scale(3.5) * Mat4::translate(0.0, -0.802, 0.0) * Mat4::rotate_x(90.0),
        Box::new(Rect::new()),
    );
    roof.material.albedo = white;
    scene.objects.push(Box::new(roof));

    // 床
    let mut floor = Entity::new(
        Mat4::scale(3.0) * Mat4::translate(0.0, 0.801, 0.0) * Mat4::rotate_x(-90.0),
        Box::new(Rect::new()),
    );
    floor.material.albedo = white;
    scene.objects.push(Box::new(floor));

    // 後ろの壁
    let mut w_back = Entity::new(
        Mat4::scale(3.0) * Mat4::translate(0.0, 0.0, 0.803) * Mat4::rotate_x(0.0),
        Box::new(Rect::new()),
    );
    w_back.material.albedo = white;
    scene.objects.push(Box::new(w_back));

    // 右の壁
    let mut w_right = Entity::new(
        Mat4::scale(3.0) * Mat4::translate(0.8, 0.0, 0.0) * Mat4::rotate_y(90.0),
        Box::new(Rect::new()),
    );
    w_right.material.albedo = Color::from_rgb(0.5, 0.0, 0.0);
    scene.objects.push(Box::new(w_right));

    // 左の壁
    let mut w_left = Entity::new(
        Mat4::scale(3.0) * Mat4::translate(-0.8, 0.0, 0.0) * Mat4::rotate_y(-90.0),
        Box::new(Rect::new()),
    );
    w_left.material.albedo = Color::from_rgb(0.0, 0.5, 0.0);
    scene.objects.push(Box::new(w_left));

    scene
}

const MIN_RAD: f32 = 0.01;

fn render(stat: &mut State, ray: &Ray, rad: f32, scene: &Scene) -> Option<Color> {
    let hit = rayhit(ray, scene);
    hit.map(|h| {
        let mat = &h.entity.material;
        let shape = &h.entity.shape;
        let local_ray = Ray::new(
            h.entity.inv_matrix * ray.origin,
            h.entity.inv_nt_matrix * ray.dir,
        );
        let local_at = local_ray.at(h.t);
        let local_normal = shape.normal(local_at);
        let at = ray.at(h.t);
        let normal = (h.entity.nt_matrix * local_normal).normalized();

        let mut albedo = Color::new();
        if rad > MIN_RAD {
            let div = ((rad / MIN_RAD) as i32).min(64).max(2);
            let div_rad = rad / (div as f32);

            for _ in 0..div {
                let x = stat.rng.gen_range(-1.0, 1.0);
                let y = stat.rng.gen_range(-1.0, 1.0);
                let z = stat.rng.gen_range(-1.0, 1.0);
                let normal = (normal + Vec3::xyz(x, y, z)).normalized();
                //let normal = Vec3::xyz(x,y,z).normalized();
                let next_ray = Ray::new(at, normal);
                let c = render(stat, &next_ray, div_rad, scene);
                match c {
                    Some(cc) => albedo = albedo + cc,
                    _ => {}
                }
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
            let local_ray = Ray::new(obj.inv_matrix * ray.origin, obj.inv_nt_matrix * ray.dir);
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
