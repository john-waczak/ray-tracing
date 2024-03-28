use super::vec::{Vec3, Point3};
use super::ray::Ray;
use super::material::Scatter;
use std::rc::Rc;


/// A struct storing data for ray-object intersection
pub struct HitRecord{
    pub p: Point3,             // intersection point
    pub normal: Vec3,          // outward surface normal
    pub mat: Rc<dyn Scatter>,  // Many records can hit the same object, so we need a reference counter. Make it work for any struct implementing Scatter trait
    pub t: f64,                // intersection "time"
    pub front_face: bool,
}


/// implementation for HitRecord
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            (-1.0) * outward_normal
        };
    }
}


/// A trait for all "hitabale" objects
pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// A dynamic list of objects implementing the "Hit" trait
pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}

