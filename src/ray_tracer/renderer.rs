use ndarray::{Array, Array3};
use rand::random;
use std::{
    array,
    f32::INFINITY,
    sync::{
        atomic::{AtomicI32, AtomicU32},
        Arc, Mutex, RwLock,
    },
    thread::{self, current},
};

use crate::{
    hittable::Hittable,
    io,
    material::ScatterResult,
    math::{Color, Ray},
    ray_tracer::renderer,
    scene::Scene,
};

pub struct Screen {
    width: usize,
    height: usize,
}

impl Screen {
    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
    pub fn new(width: usize, height: usize) -> Screen {
        Screen { width, height }
    }
    pub fn order_to_coord(&self, order: i32) -> (u32, u32) {
        let y = (order / self.width as i32 - 1).max(0) as u32;
        let x = (order % self.width as i32 - 1).max(0) as u32;

        (x, y)
    }
    pub fn new_zeros(&self) -> Array3<u8> {
        Array::zeros((self.height, self.width, 3))
    }
}

pub struct MultiThreadRenderer {
    core: RenderCore,
    array: Arc<Mutex<Array3<u8>>>,
}

pub struct RenderCore {
    screen: Screen,
    sample_per_pixel: u32,
    max_depth: u32,
    scene: Scene,
}

impl RenderCore {
    pub fn new(screen: Screen, sample_per_pixel: u32, max_depth: u32, scene: Scene) -> RenderCore {
        RenderCore {
            screen,
            sample_per_pixel,
            max_depth,
            scene,
        }
    }

    pub fn convert_color(&self,color: Color) -> [u8; 3] {
        let scale = 1.0 / self.sample_per_pixel as f32;
        let mut ouput: [u8; 3] = [0; 3];
        for i in 0..3 {
            let color_arr = color.to_array();
            ouput[i] = ((color_arr[i] * scale).powf(1.0 / 2.2) * 256.0).clamp(0.0, 255.0) as u8;
        }
        ouput
    }

    pub fn ray_color(&self, r: &Ray, world: &impl Hittable, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(1.0, 0.0, 0.0);
        }

        if let Some(hit_record) = world.hit(r, 0.001, INFINITY) {
            match hit_record.material.scatter(r, &hit_record) {
                ScatterResult::Scatterd(scattered, attenuation) => {
                    return attenuation * self.ray_color(&scattered, world, depth - 1)
                }
                ScatterResult::Stucked => {
                    return Color::new(0.0, 0.0, 1.0);
                }
                ScatterResult::None => {
                    return Color::new(0.0, 1.0, 0.0);
                }
                ScatterResult::Debug(color) => {
                    return color;
                }
            }
        }

        sky(r)
    }

    pub fn get_rendered_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let height = self.screen.height as u32;
        let y = height - y -1;
        for _ in 0..self.sample_per_pixel {
            let u = (x as f32 + random::<f32>()) / self.screen.width as f32;
            let v = (y as f32 + random::<f32>()) / self.screen.height as f32;
            color += self.ray_color(
                &self.scene.camera.get_ray(u, v),
                &self.scene.hittables,
                self.max_depth,
            );
        }

        self.convert_color(color)
    }
}

impl MultiThreadRenderer {
    pub fn new(
        screen: Screen,
        sample_per_pixel: u32,
        max_depth: u32,
        scene: Scene,
    ) -> MultiThreadRenderer {
        let array = Arc::new(Mutex::new(screen.new_zeros()));
        MultiThreadRenderer {
            core: RenderCore::new(screen, sample_per_pixel, max_depth, scene),
            array,
        }
    }

    pub fn render_screen(&mut self, t_count: u32) {
        crossbeam::scope(|scope| {
            let order = Arc::new(Mutex::new(
                self.core.screen.width as i32 * self.core.screen.height as i32,
            ));

            let core = Arc::new(&self.core);
            for i in 0..t_count {
                println!("{}", i);
                let c = Arc::clone(&core);
                let r = self.array.clone();
                let o = order.clone();
                scope.spawn(move |_| loop {
                    let mut x = 0;
                    let mut y = 0;
                    {
                        let mut current = o.lock().unwrap();
                        if *current < 0 {
                            break;
                        }

                        let pos = c.screen.order_to_coord(*current);
                        x = pos.0;
                        y = pos.1;
                        *current -= 1;

                        if *current % 5000 == 0 {
                            println!("{}", current);
                        }
                    }
                    let color = c.get_rendered_pixel(x, y);
                    let mut array = r.lock().unwrap();
                    for i in 0..3 {
                        array[[y as usize, x as usize, i]] = color[i];
                    }
                });
            }
        })
        .unwrap();
    }
    pub fn save_screen(&self, name: &str) {
        io::image::save_array_to_png(self.array.lock().unwrap().clone(), name);
    }
}

fn sky(ray: &Ray) -> Color {
    let skyblue = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);
    let unit_dir = ray.dir.to_unit();
    let t = (unit_dir.y + 1.0) / 2.0;
    ((1.0 - t) * white) + (t * skyblue)
}
