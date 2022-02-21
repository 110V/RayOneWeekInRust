use crate::math::{Vec3, Point3, Ray};


pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vfov: f32, origin: Vec3, look: Vec3, vup: Vec3) -> Camera {

        let theta = vfov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - look).to_unit();
        let u = vup.cross(w).to_unit();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2 - vertical / 2 - w;

        println!("{:#?} {:#?} {:#?} {:#?}",look,lower_left_corner,horizontal,vertical);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
