use std::sync::Arc;

use crate::math::{Point3, Vec3, Ray};
use crate::material::Material;
use super::geom::aabox::AAbox;

pub enum Face{
    Front,
    Back,
}

pub struct HitRecord{
    pub point:Point3,
    pub normal:Vec3,
    pub time:f32,
    pub face:Face,
    pub material:Arc<dyn Material>,
}

pub trait Hittable:Send+Sync{
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>;
    fn move_pos(&mut self,offset:Vec3);
    fn get_aabb(&self)->AAbox;
}

pub fn get_aabb(objects:&Vec<Box<dyn Hittable>>)->AAbox{
    let mut points = vec![];
    objects.iter().for_each(|o|{
        let aabb = o.get_aabb();
        points.push(aabb.max_p);
        points.push(aabb.min_p);
    });
    AAbox::from_points(points)
}