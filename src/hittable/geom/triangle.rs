use std::{rc::Rc, sync::Arc};

use crate::{hittable::{hittable::Face, HitRecord, Hittable}, math::{Ray, Vec3}, material::Material};

use super::{plane::Plane, aabox::AAbox};

pub enum NormalType{
    Smooth(Vec3,Vec3,Vec3),
    Single(Vec3)
}

pub struct Triangle {
    pub a:Vec3,
    pub b:Vec3,
    pub c:Vec3,
    pub normal:NormalType,
    pub area:f32,
    pub plane:Plane,
    pub mat:Arc<dyn Material>
}

impl Triangle{
    pub fn new_single(a:Vec3,b:Vec3,c:Vec3,mat:&Arc<dyn Material >)->Self{
        let plane = Plane::from_points(a,b,c);
        Triangle{
            a,b,c,normal:NormalType::Single(plane.normal),area:(a-b).cross(a-c).length()/2.0,plane,mat:mat.clone()
        }
    }
    pub fn new_smooth(a:Vec3,b:Vec3,c:Vec3,a_normal:Vec3,b_normal:Vec3,c_normal:Vec3,mat:&Arc<dyn Material >)->Self{
        let plane = Plane::from_points(a,b,c);
        Triangle{
            a,b,c,normal:NormalType::Smooth(a_normal,b_normal,c_normal),area:(a-b).cross(a-c).length()/2.0,plane,mat:mat.clone()
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

        // let x = l.cross(m).length();
        // let y = m.cross(n).length();
        // let z = n.cross(l).length();

        // (self.area*2.0 - (x+y+z)).abs()< 0.0001
        
    }

    pub fn get_barycentric(&self,p:Vec3)->Vec3{
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

        let x = l.cross(m).length();
        let y = m.cross(n).length();
        let z = n.cross(l).length();

        Vec3::new(x*a_in_out, y*b_in_out, z*c_in_out)/self.area*2.0
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

    fn move_pos(&mut self,offset:Vec3){
        self.a+=offset;
        self.b+=offset;
        self.c+=offset;
        self.plane.point+=offset;
    }

    fn get_aabb(&self)->super::aabox::AAbox {
        AAbox::from_points(vec![self.a,self.b,self.c])
    }
}