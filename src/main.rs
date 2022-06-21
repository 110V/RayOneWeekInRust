use std::{rc::Rc, sync::Arc, time::SystemTime};

use rayoneweek::{
    hittable::{
        geom::{sphere::Sphere, triangle::Triangle},
        HittableList, Hittable,
    },
    io::obj::ObjParser,
    material::{Glass, Lambertian, Material, Metal, DebugMat},
    math::{Color, Point3, Ray, Vec3},
    ray_tracer::{MultiThreadRenderer, Screen},
    scene::{Camera, Scene},
};

type RcMat = Arc<dyn Material + Send + Sync>;
fn main() {
    let red = Color::from_rgb(170, 15, 10);
    let blue = Color::from_rgb(5, 65, 180);
    let yellow = Color::from_rgb(100, 100, 0);
    let light_yellow = Color::new(1.1, 1.1, 0.5);
    let green = Color::from_rgb(12, 175, 24);
    let gray = Color::from_rgb(230, 230, 230);
    let white = Color::new(1.0, 1.0, 1.0);

    let matte_red: RcMat = Arc::new(Lambertian::new(red));
    let matte_green: RcMat = Arc::new(Lambertian::new(green));
    let matte_gray: RcMat = Arc::new(Lambertian::new(gray));
    let matte_blue: RcMat = Arc::new(Lambertian::new(blue));
    let matte_yellow: RcMat = Arc::new(Lambertian::new(yellow));

    let glass_light_yellow: RcMat = Arc::new(Lambertian::new(light_yellow));
    let glass_yellow: RcMat = Arc::new(Glass::new(yellow, 1.03));
    let metal_white: RcMat = Arc::new(Metal::new(white, 0.0));
    let debug: RcMat = Arc::new(DebugMat{albedo:white});
    let triangle = Triangle::new_single(
        Point3::new(-0.5, 0.1, -5.0),
        Point3::new(3.0, 0.1, -5.0),
        Point3::new(0.0, 0.8, -5.0),
        
        &metal_white,
    );
    
    let left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, &matte_blue);
    let right = Sphere::new(Point3::new(1.1, 0.0, -2.0), 0.5, &matte_red);
    let front = Sphere::new(Point3::new(0.0, 0.5, -1.5), 0.5, &matte_green);
    let ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, &matte_gray);
    let mut ame = ObjParser::load("smol.obj",&glass_light_yellow);
    let mut taraq = ObjParser::load("tara2.obj",&glass_yellow);

    taraq.move_pos(Vec3::new(-2.0,-0.5,1.0));
    ame.move_pos(Vec3::new(2.0,-0.5,0.0));

    let mut hittable_list = HittableList::new(Vec3::new(0.0, 0.0, 0.0));
    hittable_list.add(Box::new(taraq));
    hittable_list.add(Box::new(ame));
    hittable_list.add(Box::new(front));

    hittable_list.add(Box::new(triangle));
    hittable_list.add(Box::new(left));
    hittable_list.add(Box::new(right));
    hittable_list.add(Box::new(ground));

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: usize = 1920;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: u32 = 30;
    const MAX_DEPTH: u32 = 50;
    //camera
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let origin = Vec3::new(1.0, 1.5, 3.0);
    let look = Vec3::new(0.0, 0.0, 0.0);
    let vfov = 90.0;

    let screen = Screen::new(IMG_WIDTH, IMG_HEIGHT);
    let camera = Camera::new(ASPECT_RATIO, vfov, origin, look, vup);
    let scene = Scene::new(camera, hittable_list);

    let mut renderer = MultiThreadRenderer::new(screen, SAMPLES_PER_PIXEL, MAX_DEPTH, scene);
    let now = SystemTime::now();
    renderer.render_screen(15);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}초가 걸렸습니다!", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    renderer.save_screen("output");
    // ObjParser::load("tara2.obj");
}

fn sky(ray: &Ray) -> Color {
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let unit_dir = ray.dir.to_unit();
    let t = (unit_dir.y + 1.0) / 2.0;
    ((1.0 - t) * white) + (t * skyblue)
}
