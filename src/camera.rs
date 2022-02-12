use std::cmp::Ordering;

use crate::{vec3::{Point3, Vec3}, ray::Ray};

pub struct Camera {
    origin: Point3,
    look:Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(viewport_height:f32,viewport_width:f32,origin:Vec3,look:Vec3,vup:Vec3) ->Camera{

        let focal_length: f32 = 1.0;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0.0, 0.0, focal_length);

        let w = origin - look;
        let u = vup.cross(w).to_unit();
        let v = w.cross(u);

        Camera{
            origin,
            look,
            lower_left_corner,
            horizontal,
            vertical
        }
    }
    pub fn get_ray(&self,u:f32,v:f32)->Ray{
        Ray::new(self.origin,self.lower_left_corner+u*self.horizontal+v*self.vertical)
    }
}
