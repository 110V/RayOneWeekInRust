use crate::{vec3::Vec3, ray::Ray};

pub struct Plane{
    pub p:Vec3,
    pub normal:Vec3,
}

impl Plane{
    pub fn intersect(&self,ray:&Ray)->Option<f32>{
        let temp = ray.dir.dot(self.normal);
        if temp>0.0{
            return Some(self.normal.dot(self.p-ray.origin)/temp)
        }
        None
    }
    pub fn fromPoints(a:Vec3,b:Vec3,c:Vec3)->Plane{
        let p = a;
        let normal = (a-b).cross(a-c).to_unit();
        Plane{p,normal}
    }
}