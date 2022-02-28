use crate::math::{Vec3,Ray};

pub struct Plane{
    pub point:Vec3,
    pub normal:Vec3,
}

impl Plane{
    pub fn intersect(&self,ray:&Ray)->Option<f32>{
        let temp = ray.dir.dot(self.normal);
        if temp!=0.0{
            return Some(self.normal.dot(self.point-ray.origin)/temp)
        }
        None
    }
    pub fn from_points(a:Vec3,b:Vec3,c:Vec3)->Plane{
        let point = a;
        let normal = (a-b).cross(a-c).to_unit();
        Plane{point,normal}
    }
}