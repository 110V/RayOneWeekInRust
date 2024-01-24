use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use crate::hittable::HittableList;
use crate::hittable::geom::triangle::Triangle;
use crate::material::Material;
use crate::math::Vec3;

pub struct ObjParser {}

impl ObjParser {
    pub fn load(path: &str,mat:&Arc<dyn Material>)->HittableList {
        let input = BufReader::new(File::open(path).unwrap());
        let model: Obj = load_obj(input).unwrap();
        let mut triangle_list = HittableList::new(Vec3::new(0.0, 0.0, 0.0));
        // Do whatever you want
        //println!("{:#?}", model);

        let mut indices = model.indices;
        let len = indices.len();
        let mut get_new = ||{
            model.vertices[indices.pop().unwrap() as usize]
        };
        
        for _ in 0..len as u32 / 3 {
            let a = get_new();
            let b = get_new();
            let c = get_new();
            let a_pos = Vec3::from_array(a.position);
            let b_pos = Vec3::from_array(b.position);
            let c_pos = Vec3::from_array(c.position);
            let a_normal = Vec3::from_array(a.normal);
            let b_normal = Vec3::from_array(b.normal);
            let c_normal = Vec3::from_array(c.normal);

            let triangle = Triangle::new_smooth(c_pos, b_pos, a_pos, c_normal, b_normal, a_normal, mat);
            triangle_list.add(Box::new(triangle));
        }
        triangle_list
    }
}
