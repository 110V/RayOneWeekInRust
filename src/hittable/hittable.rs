use std::rc::Rc;

use crate::math::{Point3, Vec3, Ray};
use crate::material::Material;

pub enum Face{
    Front,
    Back,
}

pub struct HitRecord{
    pub point:Point3,
    pub normal:Vec3,
    pub time:f32,
    pub face:Face,
    pub material:Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>;
}