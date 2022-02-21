use crate::hittable::{HittableList, Hittable, self};

use super::Camera;

pub struct Scene<'a>{
    pub hittables:HittableList<'a>,
    pub camera:Camera,
}

impl<'a> Scene<'a>{
    pub fn new(camera:Camera,hittables:HittableList)->Scene{
        Scene{camera,hittables}
    }
    pub fn add_hitable(&mut self,hittable:impl Hittable + 'a){
        self.hittables.add(hittable);
    }
}