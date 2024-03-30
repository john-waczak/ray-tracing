use super::vec::{Vec3, Point3, Color};
use super::ray::Ray;
use super::hit::HitRecord;


pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian {
            albedo: a
        }
    }
}


impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        // compute direction of scattered ray
        let scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized;

        // form new ray along scattered direction
        let scattered = Ray::new(rec.p, scatter_direction);

        // return result
        Some((self.albedo, scattered))
    }
}
