use crate::{vec3::Vec3, ray::Ray};

pub struct Plane{
    p:Vec3,
    normal:Vec3,
}

impl Plane{
    pub fn intersect(&self,ray:Ray)->f32{
        self.normal.dot(self.p-ray.origin)/ray.dir.dot(self.normal)
    }
}