mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use rand::Rng;

use std::io::{stderr, Write};




/// return background color
fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec)  = world.hit(r, 0.0, f64::INFINITY) {
        // if intersection, color as sphere
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        // if no intersection, color as background
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 -t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;

    // WORLD
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // CAMERA
    let cam = Camera::new();

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255"); // rgb values in [0, 255]

    let mut rng = rand::thread_rng();


    // rows are written out
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();

        // pixels are written as rows from left to right
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("\nDone.");
}
