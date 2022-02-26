use crate::hittable::{HittableList, Hittable, self};

use super::Camera;

pub struct Scene{
    pub hittables:HittableList,
    pub camera:Camera,
}

impl Scene{
    pub fn new(camera:Camera,hittables:HittableList)->Scene{
        Scene{camera,hittables}
    }
    pub fn add_hitable(&mut self,hittable:Box<dyn Hittable+Send+Sync>){
        self.hittables.add(hittable);
    }
}