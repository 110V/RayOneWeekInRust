use crate::vec3::{Vec3, Point3};

pub struct Ray{
    pub origin:Point3,
    pub dir:Vec3,
}


impl Ray{
    pub fn new(origin:Point3,dir:Vec3)->Ray{
        Ray{origin,dir:dir}
    }
    pub fn at(&self,time:f32)->Point3{
        self.origin + self.dir * time
    }
}
