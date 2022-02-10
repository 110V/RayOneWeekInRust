use std::ptr::NonNull;

pub fn quadratic_equation(a:f32,b:f32,c:f32)->Option<[f32;2]>{
    let disc = discriminant(a, b, c);
    if disc<0.0 {
        None
    }else{
        Some([(-b-disc.sqrt())/2.0*a,(-b+disc.sqrt())/2.0*a])
    }
}

pub fn discriminant(a:f32,b:f32,c:f32)->f32{
    b*b-4.0*a*c
}