use std::{rc::Rc, sync::Arc};

use crate::{hittable::{hittable::Face, HitRecord, Hittable}, math::{Ray, Vec3}, material::Material};

use super::plane::Plane;

pub struct Triangle {
    pub a:Vec3,
    pub b:Vec3,
    pub c:Vec3,
    pub area:f32,
    pub plane:Plane,
    pub mat:Arc<dyn Material +Send +Sync>
}

impl Triangle{
    pub fn new(a:Vec3,b:Vec3,c:Vec3,mat:&Arc<dyn Material +Send +Sync >)->Self{
        Triangle{
            a,b,c,area:(a-b).cross(a-c).length()/2.0,plane:Plane::from_points(a,b,c),mat:mat.clone()
        }
    }

    pub fn plane(&self)->Plane{
        Plane::from_points(self.a,self.b,self.c)
    }

    pub fn is_in_triangle(&self,p:Vec3)->bool{
        let l = p-self.a;
        let m = p-self.b;
        let n = p-self.c;

        let u = m.cross(n).to_unit();
        let v = n.cross(l).to_unit();
        let w = l.cross(m).to_unit(); 

        let plane_normal = self.plane.normal;
        let a_in_out = u.dot(plane_normal);
        let b_in_out = v.dot(plane_normal);
        let c_in_out = w.dot(plane_normal);

        return a_in_out>0.0&&b_in_out>0.0&&c_in_out>0.0;

        let x = l.cross(m).length();
        let y = m.cross(n).length();
        let z = n.cross(l).length();

        (self.area*2.0 - (x+y+z)).abs()< 0.0001
        
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(time) = self.plane.intersect(ray){
            let point = ray.at(time);
            if t_min >= time || time >= t_max {
                return None;
            }
            if self.is_in_triangle(point){
                let mut normal = self.plane.normal;
                let mut face = Face::Front;
                
                if ray.dir.dot(normal)>0.0 {
                    normal = -1.0*normal;
                    face = Face::Back;
                    
                }
                
                let hit_record = HitRecord {
                    point,
                    normal,
                    time,
                    face,
                    material:self.mat.clone()
                };
                return Some(hit_record)
            }
        }
        None
    }
}