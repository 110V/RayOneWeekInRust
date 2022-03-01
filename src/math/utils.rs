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



#[cfg(test)]
mod test{
    use super::quadratic_equation;

    #[test]
    fn test(){
        assert_eq!(quadratic_equation(2.0,-12.0,10.0),Some([1.0,5.0]))
    }
}