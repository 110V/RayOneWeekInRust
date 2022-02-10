use ::image::math;
use num::{ToPrimitive, range};
use rayoneweek::{image, ray::Ray, vec3::{Color, Point3, Vec3}, math_utils::quadratic_equation};
use ndarray::{Array3, Array};
use rayoneweek::vec3;

fn ray_color(r:&Ray)->Color{
    let unit_dir = r.dir.to_unit();
    let t = (unit_dir.y+1.0)/2.0;
    let sphere_center = Point3::new(0.0,0.0,-1.0);
    if let Some(intersect) = ray_sphere(&r,0.5,sphere_center){
        let N =  (intersect-sphere_center).to_unit();
        return 0.5*(N+1.0);
    }
    (1.0-t)*Color::new(1.0,1.0,1.0) + t*Color::new(0.5,0.7,1.0)
}

fn ray_sphere(ray:&Ray,radius:f32,center:Point3)->Option<Vec3>{
    let radius_squared = radius*radius;
    let new_pos = ray.origin - center;
    if let Some(times) =  quadratic_equation(ray.dir.length_squared(), 2.0*new_pos.dot(ray.dir), new_pos.length_squared()-radius_squared){
        if times[0]>0.0{
            return Some(ray.at(times[0]));
        }else if times[1]>0.0 {
            return Some(ray.at(times[1]));
        }
        return None;
    }
    None
}



fn set_color(x:usize,y:usize,color:Color,arr:&mut Array3<u8>){
    let (height,width,rgb) = arr.dim();
    if rgb!=3||y>height||x>width {
        panic!("Image Index Error rgb:{} y:{} height:{} x:{} width:{}",rgb,y,height,x,width);
    }
    for i in 0..3{
        let color_arr = color.to_array();
        arr[[y,x,i]] = (color_arr[i]*255.999).clamp(0.0,255.0) as u8;
    }
}

fn main() {
    //Image
    const ASPECT_RATIO:f32  = 16.0/9.0; //width/ratio = height //height*ratio = width
    const IMG_WIDTH:usize = 400;
    const IMG_HEIGHT:usize = (IMG_WIDTH as f32/ASPECT_RATIO) as usize;


    //Camera
    let viewport_height:f32 = 2.0;
    let viewport_width:f32 = ASPECT_RATIO*viewport_height;

    let focal_length:f32 = 1.0;
    let origin:Point3 = Point3::new(0.0,0.0,0.0);

    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);
    let lower_left_corner = origin - horizontal/2 - vertical/2 - Vec3::new(0.0,0.0,focal_length);

    let mut img_array:Array3<u8> = Array::zeros((IMG_HEIGHT,IMG_WIDTH,3));

    //Render
    for y in range(0, IMG_HEIGHT){
        for x in range(0,IMG_WIDTH){
            let u = x as f32/IMG_WIDTH as f32;//horizontal viewport pos
            let v = y as f32/IMG_HEIGHT as f32;//vertical veiwport pos
            
            let ray = Ray::new(origin,lower_left_corner+u*horizontal+v*vertical - origin);

            let color = ray_color(&ray);
            set_color(x, IMG_HEIGHT-y-1, color, &mut img_array);
        }
    }

    // for ((y,x,z),v) in array.indexed_iter_mut() {
    //     *v = match z{
    //         0 => x as u8,
    //         1 => (HEIGHT - y) as u8,
    //         2 => 64,
    //         _=>unreachable!(),
    //     }
    // }


    let img = image::array_to_image(img_array);
    img.save("output.png").expect("fail");
    println!("End!");
}
