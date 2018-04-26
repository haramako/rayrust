#![allow(dead_code)]
extern crate cgmath;
extern crate getopts;
extern crate image;
extern crate rand;
extern crate rayon;

mod make_scene;
mod x3d;

use getopts::Options;
use image::{ImageBuffer, Rgba};
use rayon::prelude::*;
use std::path::Path;
use std::{env, f32};
use x3d::*;

fn main() {
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

    let scene = make_scene::make_scene();

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
