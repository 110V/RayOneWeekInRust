use std::sync::Arc;

use crate::math::Ray;

use super::{Hittable, HitRecord};

pub struct HittableList<'a>{
    pub objects:Vec<Arc<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a>{
    pub fn new()->HittableList<'a>{
        HittableList{objects:vec!()}
    }
    pub fn add(&mut self, object:impl Hittable + 'a){
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList<'_>{
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>{
        let mut result:Option<HitRecord> = None;
        let mut closest_sofar = t_max;
        self.objects.iter().for_each(|obj|{
            if let Some(hit_record) = obj.hit(ray, t_min, closest_sofar+0.001){
                closest_sofar = hit_record.time;
                result = Some(hit_record);
            }
        });
        result
    }
}