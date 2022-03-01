use std::sync::{RwLock};


use crate::math::{Ray, Vec3};

use super::{Hittable, HitRecord};

pub struct HittableList{
    pub objects:Vec<Box<dyn Hittable+Send+Sync>>,
    pub offset:Vec3,
    
}


impl<'a> HittableList{
    pub fn new(offset:Vec3)->HittableList{
        HittableList{objects:vec![],offset}
    }
    pub fn add(&mut self, object:Box<dyn Hittable+Send+Sync>){
        self.objects.push(object);
    }
}

impl Hittable for HittableList{
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

    fn move_pos(&mut self,offset:Vec3){
        self.offset+=offset;
        self.objects.iter_mut().for_each(|o|{
            o.move_pos(offset);
        })
    }
}