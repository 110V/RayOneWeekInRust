use obj::raw::object::Point;

use crate::math::{Point3, Ray, utils::overlap_range};

pub struct AABB{
    min_p:Point3,
    max_p:Point3,
}

impl AABB{
    pub fn new(min_p:Point3,max_p:Point3)->AABB{
        AABB{min_p,max_p}
    }

    pub fn fromPoints(points:Vec<Point3>)->AABB{
        if points.is_empty(){
            panic!("points is empty!")
        }
        let mut min_p = points[0].to_array();
        let mut max_p = points[0].to_array();

        for p in points{
            for i in 0..3{
                min_p[i] = min_p[i].min(p.get(i));
                max_p[i] = max_p[i].max(p.get(i));
            }
        }
        AABB::new(Point3::from_array(min_p) , Point3::from_array(max_p))
    }

    pub fn intersect(&self,ray:Ray)->Option<f32>{
        let dir = ray.dir;

        let origin = ray.origin;
        let mp_min = self.min_p-origin;
        let mp_max = self.max_p-origin;
    
        let mut ranges:Vec<(f32,f32)> = vec![]; 
    
        for i in 0..3{
            let a = mp_min.get(i);
            let b = mp_max.get(i);
            let d = dir.get(i);
            if d==0.0{
                if a*b>0.0{
                    return None;
                }
            }
            else {
                ranges.push((a/d,b/d));
    
            }
        }
        if ranges.is_empty(){
            return Some(0.0);
        }
        let mut m_range = ranges[0];
        for i in 1..ranges.len(){
            if let Some(r) = overlap_range(m_range, ranges[i]){
                m_range = r;
            }
            else{
                return None;
            }
        }
        Some(m_range.0)
    }
}

#[cfg(test)]
mod test{

    use crate::{math::Vec3, hittable::geom::AABB};

    use super::{Ray, Point3};

    #[test]
    fn test(){
        let a = AABB::AABB::new(Point3::new(-5.0, -5.0, -5.0), Point3::new(5.0, 5.0, 5.0));
        let b = AABB::AABB::new(Point3::new(-2.0, 0.0, -1.0),Point3::new(2.0, 2.0, 1.0));
        let c = AABB::AABB::new(Point3::new(-2.0, 0.0, -1.0),Point3::new(2.0, 2.0, 1.0));
        assert_eq!(a.intersect(Ray { origin: Point3::new(0.0,-10.0,0.0), dir: Vec3::new(0.0,1.0,0.0) }),Some(5.0));
        assert_eq!(b.intersect(Ray { origin: Point3::new(-5.0,-5.0,0.0), dir: Vec3::new(4.0,5.0,1.0).to_unit() }).is_some(),true);
        assert_eq!(c.intersect(Ray { origin: Point3::new(-5.0,-5.0,0.0), dir: Vec3::new(4.0,5.0,1.1).to_unit() }),None);
    }
}