use super::Vec3;

pub fn quadratic_equation(a:f32,b:f32,c:f32)->Option<[f32;2]>{
    let disc = discriminant(a, b, c);
    if disc<0.0 {
        None
    }else{
        Some([(-b-disc.sqrt())/(2.0*a),(-b+disc.sqrt())/(2.0*a)])
    }
}

pub fn discriminant(a:f32,b:f32,c:f32)->f32{
    b*b-4.0*a*c
}

pub fn overlap_range(a:(f32,f32),b:(f32,f32))->Option<(f32,f32)>{
    let result = (a.0.max(b.0),a.1.min(b.1));
    if result.0<=result.1 {
        return Some(result);
    }
    None
}



#[cfg(test)]
mod test{
    use super::{quadratic_equation, overlap_range};

    #[test]
    fn test(){
        assert_eq!(quadratic_equation(2.0,-12.0,10.0),Some([1.0,5.0]))
    }
    #[test]
    fn test_overlap(){
        assert_eq!(overlap_range((-5.0,10.0), (5.0,100.0)),Some((5.0,10.0)));
        assert_eq!(overlap_range((-5.0,10.0), (10.0,100.0)),Some((10.0,10.0)));
        assert_eq!(overlap_range((9.0,10.0), (11.0,100.0)),None);
    }
}