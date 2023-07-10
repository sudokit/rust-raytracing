// use crate::vec::{Color, Vec3};

use crate::ray::Color;
use crate::rtweekend::clamp;
use image::Rgb;

#[inline(always)]
pub fn write_to_image(pixel: &mut Rgb<u8>, color: &Color, samples_per_pixel: u32) {
    // let ir = (255.999 * color.x) as u8;
    // let ig = (255.999 * color.y) as u8;
    // let ib = (255.999 * color.z) as u8;

    // *pixel = Rgb([ir, ig, ib]);
    let mut r: f32 = color.x;
    let mut g: f32 = color.y;
    let mut b: f32 = color.z;

    let scale: f32 = 1.0 / samples_per_pixel as f32;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // idk if turning them into a u8 will introduce some bugs butt
    *pixel = Rgb([
        (256.0 * clamp(r, 0.0, 0.999)) as u8,
        (256.0 * clamp(g, 0.0, 0.999)) as u8,
        (256.0 * clamp(b, 0.0, 0.999)) as u8,
    ])
    // println!(
    //     "{} {} {}",
    //     (256.0 * clamp(r, 0.0, 0.999)) as u8,
    //     (256.0 * clamp(g, 0.0, 0.999)) as u8,
    //     (256.0 * clamp(b, 0.0, 0.999)) as u8,
    // );
}
