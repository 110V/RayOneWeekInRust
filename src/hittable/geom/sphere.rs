use std::rc::Rc;
use std::sync::Arc;

use crate::hittable::hittable::Face;
use crate::hittable::{Hittable, HitRecord};
use crate::math::utils::quadratic_equation;
use crate::math::{Point3, Ray, Vec3};
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub mat:Arc<dyn Material+Send +Sync>,
}

impl Sphere {
    pub fn new(center:Point3,radius:f32,mat:&Arc<dyn Material+Send +Sync>)->Sphere{
        Sphere{center,radius,mat:mat.clone()}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let radius_squared = self.radius * self.radius;
        let new_pos = ray.origin - self.center;
        if let Some(times) = quadratic_equation(
            ray.dir.length_squared(),
            2.0 * new_pos.dot(ray.dir),
            new_pos.length_squared() - radius_squared,
        ) {
            let mut time = times[0];
            let check_range = |time:f32|{
                t_min < time && time < t_max
            };
            if !check_range(time) {
                time = times[1];
                if !check_range(time) {
                    return None;
                }
            }
            let intersect_point = ray.at(time);
            let outer_normal = (intersect_point - self.center).to_unit();
            let mut normal = outer_normal;
            let mut face = Face::Front;
            if ray.dir.dot(outer_normal)>0.0 {
                normal = -1.0*outer_normal;
                face = Face::Back;
            }

            let hit_record = HitRecord {
                point: intersect_point,
                normal: normal,
                time: time,
                face: face,
                material:self.mat.clone()
            };
            return Some(hit_record);
        }
        None
    }

    fn move_pos(&mut self,offset:Vec3){
        self.center+=offset;
    }
}