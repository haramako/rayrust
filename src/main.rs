extern crate image;
mod x3d;

use image::{ImageBuffer, Rgba};
use std::path::Path;
use std::cmp::Ordering;

use x3d::*;

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

    let mut scene = Scene::new();

    let w: i32 = 32;
    let h: i32 = 32;

    let mut img = ImageBuffer::from_fn(w as u32, h as u32, |_, _| {
        image::Rgba([0u8, 0u8, 0u8, 255u8])
    });

    let mut s1 = Entity::new(Vec3::xyz(0.0, 0.0, 0.0), Box::new(Sphere::new(0.5)));
    s1.material.emission = Color::from_rgba(1.0, 1.0, 1.0, 1.0);
    scene.objects.push(Box::new(s1));

    let s3 = Entity::new(Vec3::xyz(0.2, 0.3, 0.3), Box::new(Sphere::new(0.7)));
    scene.objects.push(Box::new(s3));

    let mut r1 = Entity::new(Vec3::xyz(0.0, 0.0, 0.4), Box::new(Rect::new()));
    r1.material.emission = Color::from_rgba(1.0, 1.0, 0.0, 1.0);
    scene.objects.push(Box::new(r1));

    let scene = scene;

    for _x in 0..w {
        for _y in 0..h {
            let x = ((_x - w / 2) as f32) / (w as f32);
            let y = ((_y - h / 2) as f32) / (h as f32);
            let ray = Ray::new(Vec3::xyz(0.0, 0.0, -2.0), Vec3::xyz(x, y, 1.0).normalized());
            let color = render(&ray, 1.0, &scene);
            img[(_x as u32, _y as u32)] = color
                .map(|c| color_to_rgba(&c))
                .unwrap_or(Rgba([0, 0, 0, 255]));
        }
    }

    img.save(Path::new("test.png")).unwrap();
}

fn render(ray: &Ray, rad: f32, scene: &Scene) -> Option<Color> {
    let hit = scene
        .objects
        .iter()
        .flat_map(|&ref obj| obj.shape.ray_cast(&obj, &ray))
        .min_by(|a, b| f32_cmp(a.t, b.t));

    hit.map(|h| {
        let col = h.entity.material.emission;
        col * rad
    })
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
    r[0] = (c.data[0] * 255.0) as u8;
    r[1] = (c.data[1] * 255.0) as u8;
    r[2] = (c.data[2] * 255.0) as u8;
    r[3] = 255;
    Rgba(r)
}
