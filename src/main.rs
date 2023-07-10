mod camera;
mod hittable;
mod hittable_list;
mod img;

mod ray;
mod rtweekend;
mod sphere;
// mod vec;

use std::rc::Rc;

use hittable::{HitRecord, Hittable};
use image::{ImageBuffer, RgbImage};
use img::write_to_image;
use indicatif::{ProgressBar, ProgressStyle};
use nalgebra::Matrix3x5;
use ray::{Color, Point3, Ray, Vec3};
use rtweekend::{random_in_hemisphere, INFINITY};

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec: HitRecord = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(ray, 0.001, INFINITY, &mut rec) {
        let target: Point3 = rec.p + random_in_hemisphere(&rec.normal);
        // return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        return 0.5
            * ray_color(
                &Ray {
                    origin: rec.p,
                    dir: target - rec.p,
                },
                world,
                depth - 1,
            );
        // let scattered: Ray = Ray::default();
        // let atteniation: Color = Color::default();
        // if rec.mat_ptr.scatter(ray, &rec, &atteniation, &scattered) {
        //     return 0.5 * ray_color(&scattered, world, depth - 1);
        // }
        // return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = ray.dir.normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image
    const ASPECT_RATION: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATION) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    // const SAMPLES_PER_PIXEL: u32 = 2;
    const MAX_DEPTH: u32 = 50;
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // world
    let mut world: HittableList = HittableList::new();

    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    // const VIEWPORT_WIDTH: f32 = ASPECT_RATION * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;
    let camera: Camera = Camera::new(ASPECT_RATION, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // show progress
    let pb: ProgressBar = ProgressBar::new((IMAGE_WIDTH * IMAGE_HEIGHT * SAMPLES_PER_PIXEL) as u64);

    // // render
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}: [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len}",
        )
        .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("=>-"),
    );
    pb.set_message("Rendering image...");

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut color: Color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..SAMPLES_PER_PIXEL {
            let u: f32 = (x as f32 + fastrand::f32()) / (IMAGE_WIDTH - 1) as f32;
            // let u: f32 = (x as f32) / (IMAGE_WIDTH - 1) as f32;
            // 1.0 - ... because we go from top to down and the book goes from bottom to top (top being 0 and bottom the height of the image)
            let v: f32 = 1.0 - (y as f32 + fastrand::f32()) / (IMAGE_HEIGHT - 1) as f32;
            // let v: f32 = 1.0 - (y as f32) / (IMAGE_HEIGHT - 1) as f32;
            let ray: Ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, MAX_DEPTH);
            pb.inc(1);
        }
        write_to_image(pixel, &color, SAMPLES_PER_PIXEL);
    }
    pb.finish_with_message("Done rendering image");
    buffer.save("image.png").unwrap();
}
