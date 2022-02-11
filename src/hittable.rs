use crate::{vec3::{Point3, Vec3}, ray::Ray};

pub struct HitRecord{
    pub p:Point3,
    pub normal:Vec3,
    pub t:f32,
    pub front_face:bool,
}

pub trait Hittable {
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>;
}

pub struct HittableList{
    pub objects:Vec<Box<dyn Hittable>>,
}

impl HittableList{
    pub fn new()->HittableList{
        HittableList{objects:vec!()}
    }
    pub fn add(&mut self,obj:Box<dyn Hittable>){
        self.objects.push(obj);
    }
}
impl Hittable for HittableList{
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>{
        let mut tmp_record:Option<HitRecord> = None; 
        let mut closest_sofar = t_max;
        self.objects.iter().for_each(|obj|{
            if let Some(hit_record) = obj.hit(ray, t_min, closest_sofar){
                closest_sofar = hit_record.t;
                tmp_record = Some(hit_record);
            }
        });
        tmp_record
    }
}