

use super::{Ray, Point3, utils::overlap_range};

fn collide(p_min:Point3,p_max:Point3,ray:Ray)->Option<f32>{
    let dir = ray.dir;

    let origin = ray.origin;
    let mp_min = (p_min-origin);
    let mp_max = (p_max-origin);

    let mut ranges:Vec<(f32,f32)> = vec![]; 

    for i in 0..3{
        if dir.get(i)==0.0{
            let a = mp_min.get(i);
            let b = mp_max.get(i);
            if a > 0.0 || 0.0 <b{
                return None;
            }
            ranges.push((a,b))
        }
    }
    if ranges.is_empty(){
        return Some(0.0)
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
