mod camera;

pub use camera::Camera;

use crate::hittable::{Hittable, HittableList};

pub struct Scene{
    pub hittables:HittableList,
    pub camera:Camera,
}

impl Scene{
    pub fn new(camera:Camera,hittables:HittableList)->Scene{
        Scene{camera,hittables}
    }
    pub fn add_hitable(&mut self,hittable:Box<dyn Hittable>){
        self.hittables.add(hittable);
    }
}