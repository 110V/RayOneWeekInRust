use rand::random;

use crate::hittable::Face;

use crate::{
    hittable::HitRecord,
    math::{Ray,Color,Vec3},
};

pub enum ScatterResult {
    Scatterd(Ray,Color),
    None,
    Stucked,
    Debug(Color),
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> ScatterResult;
}

pub struct DebugMat {
    albedo: Color,
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

pub struct Glass {
    albedo: Color,
    refract_i: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Glass {
    pub fn new(albedo: Color, refract_i: f32) -> Glass {
        Glass { albedo, refract_i }
    }
    pub fn reflectace(cos: f32, refract_ratio: f32) -> f32 {
        let r0 = ((1.0 - refract_ratio) / (1.0 + refract_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        if let Face::Back = rec.face {
            return ScatterResult::Stucked;
        }
        let mut scatter_direction =
            rec.normal + Vec3::random_in_unit_sphere().to_unit();
        if scatter_direction.near_sero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scatter_direction);
        ScatterResult::Scatterd(scattered, self.albedo)
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        if let Face::Back = rec.face {
            return ScatterResult::Stucked;
        }
        let reflected = Vec3::reflect(r_in.dir.to_unit(), rec.normal);
        let scatterd = Ray::new(rec.point, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scatterd.dir.dot(rec.normal) < 0.0 {
            return ScatterResult::None;
        }
        ScatterResult::Scatterd(scatterd, self.albedo)
    }
}

impl Material for Glass {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {
        let mut in_n = self.refract_i;
        let mut out_n = 1.0;
        if let Face::Back = rec.face {
            in_n = 1.0;
            out_n = in_n;
        }
        let cos = r_in.dir.to_unit().dot(-1.0 * rec.normal).min(1.0);
        let mut out_dir: Vec3 = Vec3::reflect(r_in.dir, rec.normal);
        if Glass::reflectace(cos, out_n / in_n) < random() {
            if let Some(refracted) = Vec3::refract(out_n, in_n, r_in.dir.to_unit(), rec.normal) {
                out_dir = refracted;
            }
        }
        let scatterd = Ray::new(rec.point, out_dir);
        ScatterResult::Scatterd(scatterd, self.albedo)
    }
}

impl Material for DebugMat {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> ScatterResult {   
        ScatterResult::Debug(self.albedo)
    }
}
