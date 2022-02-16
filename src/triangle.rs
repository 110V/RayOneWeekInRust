use std::rc::Rc;

use crate::{vec3::Vec3, plane::{Plane}, hittable::{Hittable, HitRecord}, ray::Ray, material::Material};

pub struct Triangle {
    pub a:Vec3,
    pub b:Vec3,
    pub c:Vec3,
    pub area:f32,
    pub plane:Plane,
    pub mat:Rc<dyn Material>
    //barycenter:Vec3,
}

impl Triangle{
    pub fn new(a:Vec3,b:Vec3,c:Vec3,mat:&Rc<dyn Material>)->Self{
        Triangle{
            a,b,c,area:(a-b).cross(a-c).length()/2.0,plane:Plane::fromPoints(a,b,c),mat:mat.clone()
        }
    }

    pub fn plane(&self)->Plane{
        Plane::fromPoints(self.a,self.b,self.c)
    }

    pub fn inTriangle(&self,p:Vec3)->bool{
        let l = p-self.a;
        let m = p-self.b;
        let n = p-self.c;

        let x = l.cross(m).length();
        let y = m.cross(n).length();
        let z = n.cross(l).length();
        self.area*2.0+0.001>=x+y+z
    }


}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(time) = self.plane.intersect(ray){
            let p = ray.at(time);
            if t_min >= time || time >= t_max {
                return None;
            }
            if self.inTriangle(p){
                let mut normal = self.plane.normal;
                if ray.dir.dot(normal)>0.0 {
                    normal = -1.0*normal;
                }
                //println!("{:#?}",normal);
                let hit_record = HitRecord {
                    p,
                    normal,
                    t: time,
                    front_face:true,
                    mat:self.mat.clone()
                };
                return Some(hit_record)
            }
        }
        None
    }
}