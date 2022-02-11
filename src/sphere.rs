use crate::{
    hittable::{HitRecord, Hittable},
    math_utils::quadratic_equation,
    ray::Ray,
    vec3::Point3,
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center:Point3,radius:f32)->Sphere{
        Sphere{center,radius}
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
            let mut front_face = true;
            if ray.dir.dot(outer_normal)>0.0 {
                normal = -1.0*outer_normal;
                front_face = false;
            }

            let hit_record = HitRecord {
                p: intersect_point,
                normal: normal,
                t: time,
                front_face:front_face,
            };
            return Some(hit_record);
        }
        None
    }
}
