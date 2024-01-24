use crate::{math::{Point3, Vec3, Ray}};

use super::{Hittable, geom::aabox::AAbox, get_aabb, HitRecord};
use std::{cmp::Ordering::Equal, clone};

pub enum Child{
    Node(Box<dyn Hittable>),
    None
}

pub struct BvhNode{
    pub left:Child,
    pub right:Child,

    pub aabb:AAbox,
}

impl BvhNode{

    fn to_child(self)->Child{
        Child::Node(Box::new(self))
    }

    pub fn new(mut objs:Vec<Box<dyn Hittable>>)->Option<BvhNode>{
        if objs.is_empty(){
            return None;
        }
        let aabb = get_aabb(&objs);

        if objs.len() == 1{
            return Some(BvhNode{
                left:Child::Node(objs.remove(0)),
                right:Child::None,
                aabb
            });
        }
        if objs.len() == 2{
            return Some(BvhNode{
                left:Child::Node(objs.remove(0)),
                right:Child::Node(objs.remove(0)),
                aabb
            });
        }

        let (left,right) = Self::split_objects(objs);
        let mut l_node = Child::None;
        let mut r_node = Child::None;

        if let Some(node) = Self::new(left){
            l_node = node.to_child();
        }
        if let Some(node) = Self::new(right){
            r_node = node.to_child();
        }

        Some(BvhNode { left: l_node, right: r_node, aabb })
    }
    fn split_objects(mut objs:Vec<Box<dyn Hittable>>)->(Vec<Box<dyn Hittable>>,Vec<Box<dyn Hittable>>){
        if objs.is_empty(){
            return (vec![],vec![]);
        }
        let mut bounds:Vec<Point3> = objs.iter().enumerate().map(|(i,o)|o.get_aabb().center()).collect();
        let t_bounds = bounds.clone();
        let mut sorts:Vec<(usize,f32)> = vec![];
        
        for i in 0..3{
            bounds.sort_by(|a,b|{
                a.get(i).partial_cmp(&b.get(i))
                .unwrap_or(Equal)
            });
            let min = bounds.first().unwrap().get(i);
            let max = bounds.last().unwrap().get(i);
            sorts.push((i,max-min));
        }
        sorts.sort_by(|a,b|{
            a.1.partial_cmp(&b.1)
            .unwrap_or(Equal)
        });

        let xyz = sorts[0].0;  
        let mut zipped:Vec<(&Point3,Box<dyn Hittable>)> = t_bounds.iter().zip(objs).collect();

        
        zipped.sort_by(|a,b|{
            let a_value = a.0.get(xyz);
            let b_value = b.0.get(xyz);
            a_value.partial_cmp(&b_value)
            .unwrap_or(Equal)
        });


        let half = zipped.len()/2;
        let len = zipped.len();
        let mut left = vec![];
        let mut right = vec![];
        

        for i in 0..half{
            left.push(zipped.remove(0).1);
        }
        for i in half..len{
            right.push(zipped.remove(0).1);
        }
        (left,right)
        
    }
}

impl Hittable for BvhNode{
    fn hit(&self,ray:&Ray,t_min:f32,t_max:f32)->Option<HitRecord>{
        let mut result:Option<HitRecord> = None;
        if self.aabb.intersect(ray).is_some(){

            if let Child::Node(hittable) = &self.left{
                result = hittable.hit(ray, t_min, t_max);
            }

            if let Child::Node(hittable) = &self.right{
                let hr = hittable.hit(ray, t_min, t_max);
                if let Some(hit_record) = &hr {

                    if let Some(result_hit) = &result{
                        if hit_record.time<result_hit.time{
                            result = hr;
                        }
                    }

                    else{
                        result = hr;
                    }

                }
            }
        }
        result
    }
    fn move_pos(&mut self,offset:Vec3){
        fn move_child(child:&mut Child,offset: Vec3){
            if let Child::Node(hittable) = child{
                hittable.move_pos(offset);
            }
        }
        move_child(&mut self.left,offset);
        move_child(&mut self.right, offset)
    }
    fn get_aabb(&self)->AAbox{
        self.aabb
    }
}

