use std::{f32::INFINITY, collections::btree_map::Range};

use ndarray::Array3;
use rand::random;

use crate::{scene::Scene, math::{Color, Ray}, hittable::Hittable, material::ScatterResult};

pub struct Screen{
    width:u32,
    height:u32,
}

impl Screen{
    pub fn aspect_ratio(&self)->f32{
        self.width as f32/self.height as f32
    }
    pub fn new(width:u32,height:u32)->Screen{
        Screen{
            width,
            height
        }
    }
}


pub struct Renderer<T:Fn(&Ray)->Color + Send + Sync>{
    screen:Screen,
    sample_per_pixel:u32,
    env_map:T,
    scene:Scene,
    array:Array3<u8>,//mutex need
}


impl<T:Fn(&Ray)->Color + Send + Sync> Renderer<T> {
    pub fn set_color(&mut self,x: u32, y: u32, color: Color) {
        let x = x as usize;
        let y = y as usize;
        let scale = 1.0/self.sample_per_pixel as f32;
        for i in 0..3 {
            let color_arr = color.to_array();
            self.array[[y, x, i]] = ((color_arr[i]* scale).powf(1.0/2.2) * 256.0).clamp(0.0, 255.0) as u8;
        }
    }

    pub fn ray_color(&self,r: &Ray, world: &impl Hittable,depth:usize) -> Color {
        let unit_dir = r.dir.to_unit();
        
        
        if depth<=0{
            return Color::new(1.0, 0.0, 0.0)
        }

        if let Some(hit_record) = world.hit(r, 0.001, INFINITY) {
            match hit_record.material.scatter(r, &hit_record){
                ScatterResult::Scatterd(scattered,attenuation)=>{
                    return attenuation * self.ray_color(&scattered,world,depth-1)
                },
                ScatterResult::Stucked=>{
                    return Color::new(0.0, 0.0, 1.0);
                },
                ScatterResult::None=>{
                    return Color::new(0.0, 1.0, 0.0);
                },
                ScatterResult::Debug(color)=>{
                    return color;
                }
            }
        }
        // let bias = (unit_dir.y + 1.0) / 2.0;
        // (1.0 - bias) * Color::new(1.0, 1.0, 1.0) + bias * Color::new(0.5, 0.7, 1.0)
        (self.env_map)(r)
    }

    pub fn render_pixel(&mut self,x:u32,y:u32){
        let mut color = Color::new(0.0,0.0,0.0);
        for _ in 0..self.sample_per_pixel {
            let u = (x as f32 + random::<f32>()) / self.screen.width as f32;
            let v = (y as f32 + random::<f32>()) / self.screen.height as f32;
            color += self.ray_color(&mut self.scene.camera.get_ray(u, v), &self.scene.hittables,50);
        } 
        self.set_color(x, self.screen.height  - y - 1, color);
    }
}

