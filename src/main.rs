mod vec;
mod ray;

use vec::{Vec3, Point3, Color};
use ray::Ray;
use std::io::{stderr, Write};


/// check if ray has hit sphere
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;

    let a: f64 = r.direction().dot(r.direction());
    let b: f64 = 2.0 * oc.dot(r.direction());
    let c: f64 = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}



/// return background color
fn ray_color(r: &Ray) -> Color {
    // check intersection with sphere of radius 0.5 at z=-1.0
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - Point3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().normalized();

    // convert ray height from [-1,1] --> [0, 1]
    let t = 0.5 * (unit_direction.y() + 1.0);

    // interpolate between blue and white
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;


    // CAMERA
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0; // distance from viewport to camera

    // x, y, z
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);


    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255"); // rgb values in [0, 255]

    // rows are written out
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j);
        stderr().flush().unwrap();

        // pixels are written as rows from left to right
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + (u*horizontal) + (v * vertical) - origin,
            );

            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }

    eprintln!("\nDone.");
}
