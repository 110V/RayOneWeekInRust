// use std::{f32::INFINITY, rc::Rc};

// use ndarray::{Array, Array3};

// use rand::prelude::*;
// use rayoneweek::{math::{Ray, Color, Vec3}, hittable::Hittable, material::ScatterResult, scene::Camera};

// fn ray_color(r: &Ray, world: &impl Hittable,depth:usize) -> Color {
//     let unit_dir = r.dir.to_unit();
//     let t = (unit_dir.y + 1.0) / 2.0;
//     if depth<=0{
//         return Color::new(0.0, 0.0, 0.0)
//     }
//     // r.origin = r.origin.forward(r.dir, 0.0);
//     if let Some(hit_record) = world.hit(r, 0.001, INFINITY) {
//         match hit_record.material.scatter(r, &hit_record){
//             ScatterResult::Scatterd(scattered,attenuation)=>{
//                 return attenuation * ray_color(&scattered,world,depth-1)
//             },
//             ScatterResult::Stucked=>{
//                 return Color::new(0.0, 0.0, 1.0);
//             },
//             ScatterResult::None=>{
//                 return Color::new(0.0, 0.0, 1.0);
//             },
//             ScatterResult::Debug(color)=>{
//                 return color;
//             }
//         }
//     }
//     (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
// }

// fn set_color(x: usize, y: usize, color: Color, arr: &mut Array3<u8>,samples_per_pixel:u32) {
//     let (height, width, rgb) = arr.dim();
//     if rgb != 3 || y > height || x > width {
//         panic!(
//             "Image Index Error rgb:{} y:{} height:{} x:{} width:{}",
//             rgb, y, height, x, width
//         );
//     }
//     let scale = 1.0/samples_per_pixel as f32;
//     for i in 0..3 {
//         let color_arr = color.to_array();
//         arr[[y, x, i]] = ((color_arr[i]* scale).powf(1.0/2.2) * 256.0).clamp(0.0, 255.0) as u8;
//     }
// }

// fn main() {
//     //Image
//     let split_count = 1;
//     let mut line = String::new();
//     std::io::stdin().read_line(&mut line).unwrap();
//     println!("Enter image num :{}",line);
//     let num:usize = line.trim().parse().unwrap();
//     println!("Enter image num :{}",&num);
//     const ASPECT_RATIO: f32 = 16.0 / 9.0; //width/ratio = height //height*ratio = width
//     const IMG_WIDTH: usize = 1080;
//     const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
//     const SAMPLES_PER_PIXEL:u32 = 100;
//     let vup = Vec3::new(0.0,1.0,0.0);
//     let origin = Vec3::new (0.0,1.5,1.0);
//     let look = Vec3::new (0.0,0.5,-1.0);
//     let vfov = 90.0;
//     let cam:Camera = Camera::new(ASPECT_RATIO,vfov,origin,look,vup);

//     let mut img_array: Array3<u8> = Array::zeros((IMG_HEIGHT/split_count, IMG_WIDTH, 3));

//     let mut world = HittableList::new();

//     let debug_mat:Rc<dyn Material> = Rc::new(DebugMat{albedo:vup});
//     let material_ground:Rc<dyn Material>  = Rc::new(Lambertian::new(Color::new(0.5, 0.7, 0.7)));
//     let material_center:Rc<dyn Material> = Rc::new(Glass::new(Color::new(1.0, 1.0, 1.0),1.03));
//     let material_left:Rc<dyn Material> = Rc::new(Metal::new(Color::new(1.0, 1.0, 1.0),0.0));
//     let material_right:Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.6, 0.8)));
//     let material_front:Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
//     let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
//     let center = Sphere::new(Point3::new(0.0, 1.0, -1.0), 0.5,&material_center);
//     let triangle = Triangle::new(
//         Point3::new(0.0, 0.8, -3.0),
//         Point3::new(3.0, 0.1, -3.0),
//         Point3::new(-0.5, 0.1, -3.0),&material_left);
//     let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5,&material_left);
//     let right = Sphere::new(Point3::new(1.5, 0.0, -1.0), 0.5,&material_right);
//     let front = Sphere::new(Point3::new(0.0,-0.5,-1.5),0.1,&material_front);

//     world.add(Box::new(center));
//     world.add(Box::new(ground));
//     world.add(Box::new(left));
//     world.add(Box::new(right));
//     world.add(Box::new(triangle));


//     //world.add(Box::new(front));
    
//     //Render
//     for y in range(IMG_HEIGHT/split_count*num, IMG_HEIGHT/split_count*num+IMG_HEIGHT/split_count) {
//         println!("{}",IMG_HEIGHT-y);
//         for x in range(0, IMG_WIDTH) {
//             let mut color = Color::new(0.0,0.0,0.0);
//             for _ in range(0,SAMPLES_PER_PIXEL){
//                 let u = (x as f32 + random::<f32>()) / IMG_WIDTH as f32; //horizontal viewport pos + random::<f32>()
//                 let v = (y as f32 + random::<f32>()) / IMG_HEIGHT as f32; //vertical veiwport pos
//                 color += ray_color(&mut cam.get_ray(u, v), &world,50);
//             } 
//             set_color(x, IMG_HEIGHT/split_count + IMG_HEIGHT/split_count*num - y - 1, color, &mut img_array,SAMPLES_PER_PIXEL);

//         }
//     }

//     let img = image::array_to_image(img_array);
//     img.save(format!("output{}.png",num)).expect("fail");
//     println!("End!");
// }

use std::{rc::Rc, sync::Arc, time::SystemTime};

use rayoneweek::{ray_tracer::{MultiThreadRenderer, Screen}, math::{Color, Ray, Point3, Vec3}, hittable::{HittableList, geom::{triangle::Triangle, sphere::Sphere}}, material::{Material, Lambertian, Glass, Metal}, scene::{Scene, Camera}};


type RcMat = Arc<dyn Material + Send + Sync>;
fn main(){
    let red = Color::from_rgb(170,15,10);
    let blue = Color::from_rgb(5,65,180);
    let green = Color::from_rgb(12,175,24);
    let gray = Color::from_rgb(230,230,230);
    let white = Color::new(1.0, 1.0, 1.0);

    let matte_red:RcMat  = Arc::new(Lambertian::new(red));
    let matte_green:RcMat = Arc::new(Lambertian::new(green));
    let matte_gray:RcMat = Arc::new(Lambertian::new(gray));
    let matte_blue:RcMat = Arc::new(Lambertian::new(blue));

    let glass_white:RcMat = Arc::new(Glass::new(white,1.03));
    let metal_white:RcMat = Arc::new(Metal::new(white,0.0));

    let triangle = Triangle::new(
        Point3::new(0.0, 0.8, -3.0),
        Point3::new(3.0, 0.1, -3.0),
        Point3::new(-0.5, 0.1, -3.0),&metal_white);
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5,&metal_white);
    let right = Sphere::new(Point3::new(1.5, 0.0, -1.0), 0.5,&matte_red);
    let front = Sphere::new(Point3::new(0.0,-0.5,-1.5),0.1,&glass_white);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &matte_green);

    let mut hittable_list= HittableList::new();
    hittable_list.add(Box::new(front));

    hittable_list.add(Box::new(triangle));
    hittable_list.add(Box::new(left));
    hittable_list.add(Box::new(right));
    hittable_list.add(Box::new(ground));




    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: usize = 1920;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:u32 = 1000;
    const MAX_DEPTH:u32 = 50;
    //camera
    let vup = Vec3::new(0.0,1.0,0.0);
    let origin = Vec3::new (0.0,1.5,1.0);
    let look = Vec3::new (0.0,0.5,-1.0);
    let vfov = 90.0;

    let screen = Screen::new(IMG_WIDTH,IMG_HEIGHT);
    let camera = Camera::new(ASPECT_RATIO, vfov, origin, look, vup);
    let scene = Scene::new(camera,hittable_list);
    

    let mut renderer = MultiThreadRenderer::new(screen,SAMPLES_PER_PIXEL,MAX_DEPTH,scene);
    let now = SystemTime::now();
    renderer.render_screen(30);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}초가 걸렸습니다!", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    renderer.save_screen("output");
}

fn sky(ray:&Ray)->Color{
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let unit_dir = ray.dir.to_unit();
    let t = (unit_dir.y + 1.0) / 2.0;
    ((1.0 - t) * white) + (t * skyblue)
}