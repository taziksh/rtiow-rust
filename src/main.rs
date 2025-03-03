mod vec3;
mod ray;
mod color;
mod point3;
mod constants;
mod hittable;
mod hittable_list;
mod sphere;

use crate::ray::Ray;
use crate::color::Color;
use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::constants::*;
use crate::point3::Point3;

use log::info;
use env_logger;
use std::io::Write;

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(rec) = world.hit(ray, 0.0, infinity) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    } 

    let unit_direction = Vec3::unit_vector(&ray.direction()); // [-1, 1]
    let a = 0.5 * (unit_direction.y() + 1.0); // [0, 2] -> [0, 1]
    
    // linear interpolation "switch": 
    // 1.0 is white, 0.0 is blue
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;

    let mut image_height = image_width / aspect_ratio;
    if image_height < 1.0 {image_height = 1.0};

    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5)
    ));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0
    )));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let focal_length = 1.0;

    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); // 0.5 inset on both sides for start/end symmetry 

    // Rendering
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height as u16 {
        info!("\rScanlines remaining: {} ", image_height as u16 - j);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width as u16 {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray, &world);
            color::write_color(&mut std::io::stdout(), pixel_color);
        }
    }
    info!("\rDone.          \n");
}
