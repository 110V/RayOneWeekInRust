use std::{f32::INFINITY, rc::Rc};

use ndarray::{Array, Array3};
use num::{range, traits::Pow};
use rand::{distributions::Uniform, prelude::Distribution};
use rayoneweek::{
    hittable::{Hittable, HittableList},
    image,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point3, Vec3}, camera::Camera, material::{Lambertian, Metal, Material, Glass},
};

fn ray_color(r: &Ray, world: &impl Hittable,depth:usize) -> Color {
    let unit_dir = r.dir.to_unit();
    let t = (unit_dir.y + 1.0) / 2.0;
    if depth<=0{
        return Color::new(0.0, 0.0, 0.0)
    }
    if let Some(hit_record) = world.hit(r, 0.001, INFINITY) {
        if let Some((scattered,attenuation )) = hit_record.mat.scatter(r, &hit_record){
            return attenuation * ray_color(&scattered,world,depth-1)
        }
        else{
            return r.dir*0.5 + 0.5 //Color::new(0.0, 0.0, 0.0)
        }
    }
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn set_color(x: usize, y: usize, color: Color, arr: &mut Array3<u8>,samples_per_pixel:u32) {
    let (height, width, rgb) = arr.dim();
    if rgb != 3 || y > height || x > width {
        panic!(
            "Image Index Error rgb:{} y:{} height:{} x:{} width:{}",
            rgb, y, height, x, width
        );
    }
    let scale = 1.0/samples_per_pixel as f32;
    for i in 0..3 {
        let color_arr = color.to_array();
        arr[[y, x, i]] = ((color_arr[i]* scale).pow(1.0/2.2) * 256.0).clamp(0.0, 255.0) as u8;
    }
}

fn main() {
    //Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0; //width/ratio = height //height*ratio = width
    const IMG_WIDTH: usize = 400;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL:u32 = 100;

    let cam:Camera = Camera::new(2.0,2.0*ASPECT_RATIO);

    let mut img_array: Array3<u8> = Array::zeros((IMG_HEIGHT, IMG_WIDTH, 3));

    let mut world = HittableList::new();

    let material_ground:Rc<dyn Material>  = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center:Rc<dyn Material> = Rc::new(Glass::new(Color::new(1.0, 1.0, 1.0),1.3));
    let material_left:Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.7, 1.0, 0.7)));
    let material_right:Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.6, 0.2)));
    let material_front:Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &material_ground);
    let center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5,&material_center);
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5,&material_left);
    let right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5,&material_right);
    let front = Sphere::new(Point3::new(0.0,-0.5,-1.5),0.1,&material_front);

    world.add(Box::new(ground));
    world.add(Box::new(center));
    world.add(Box::new(left));
    world.add(Box::new(right));
    world.add(Box::new(front));
    let mut rng = rand::thread_rng();
    let rand_range = Uniform::new(0.0f32,1.0f32);
    
    //Render
    for y in range(0, IMG_HEIGHT) {
        println!("{}",y);
        for x in range(0, IMG_WIDTH) {
            let mut color = Color::new(0.0,0.0,0.0);
            for s in range(0,SAMPLES_PER_PIXEL){
                let u = (x as f32 + rand_range.sample(&mut rng)) / IMG_WIDTH as f32; //horizontal viewport pos
                let v = (y as f32  + rand_range.sample(&mut rng)) / IMG_HEIGHT as f32; //vertical veiwport pos
                color += ray_color(&cam.get_ray(u, v), &world,50);
            } 
            set_color(x, IMG_HEIGHT - y - 1, color, &mut img_array,SAMPLES_PER_PIXEL);

        }
    }

    let img = image::array_to_image(img_array);
    img.save("output.png").expect("fail");
    println!("End!");
}
