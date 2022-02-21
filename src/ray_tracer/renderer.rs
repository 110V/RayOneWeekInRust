use std::{f32::INFINITY};
use ndarray::{Array, Array3};
use rand::random;

use crate::{scene::Scene, math::{Color, Ray}, hittable::Hittable, material::ScatterResult, io};

pub struct Screen{
    width:usize,
    height:usize,
}

impl Screen{
    pub fn aspect_ratio(&self)->f32{
        self.width as f32/self.height as f32
    }
    pub fn new(width:usize,height:usize)->Screen{
        Screen{
            width,
            height
        }
    }
}


pub struct Renderer<'a,T:Fn(&Ray)->Color + Send + Sync>{
    screen:Screen,
    sample_per_pixel:u32,
    max_depth:u32,
    env_map:T,
    scene:Scene<'a>,
    array:Array3<u8>,//mutex need
}


impl<T:Fn(&Ray)->Color + Send + Sync> Renderer<'_,T> {
    pub fn new(screen:Screen,sample_per_pixel:u32,max_depth:u32,env_map:T,scene:Scene)->Renderer<'_,T>{
        let array:Array3<u8> = Array::zeros((screen.height, screen.width, 3));
        Renderer{screen,sample_per_pixel,max_depth,env_map,scene,array}
    }

    pub fn set_color(&mut self,x: u32, y: u32, color: Color) {
        let x = x as usize;
        let y = y as usize;
        let scale = 1.0/self.sample_per_pixel as f32;
        for i in 0..3 {
            let color_arr = color.to_array();
            self.array[[y, x, i]] = ((color_arr[i]* scale).powf(1.0/2.2) * 256.0).clamp(0.0, 255.0) as u8;
        }
    }

    pub fn ray_color(&self,r: &Ray, world: &impl Hittable,depth:u32) -> Color {
        
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

        (self.env_map)(r)
    }

    pub fn render_pixel(&mut self,x:u32,y:u32){
        let mut color = Color::new(0.0,0.0,0.0);
        for _ in 0..self.sample_per_pixel {
            let u = (x as f32 + random::<f32>()) / self.screen.width as f32;
            let v = (y as f32 + random::<f32>()) / self.screen.height as f32;
            color += self.ray_color(&mut self.scene.camera.get_ray(u, v), &self.scene.hittables,self.max_depth);
        } 
        self.set_color(x, self.screen.height as u32  - y - 1, color);
    }

    pub fn render_screen(&mut self){
        for y in 0..self.screen.height {
            println!("{}",self.screen.height-y);
            for x in 0..self.screen.width {
                self.render_pixel(x as u32, y as u32);
            }
        }
    }

    pub fn save_screen(&self,name:&str){
        io::image::save_array_to_png(self.array.clone(), name);
    }
}

