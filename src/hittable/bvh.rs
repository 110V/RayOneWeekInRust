use crate::math::Point3;

use super::{Hittable, geom::aabox::AAbox, hittable::get_aabb};
use std::cmp::Ordering::Equal;

pub enum Child{
    Node(Box<BvhNode>),
    Hittable(Box<dyn Hittable>),
    None
}

pub struct BvhNode{
    pub left:Child,
    pub right:Child,

    pub aabb:AAbox,
}

impl BvhNode{
    pub fn new(mut objs:Vec<Box<dyn Hittable>>)->Option<BvhNode>{
        let child = Self::get_child(objs);
        if let Child::Node(node) = child{
            return Some(*node);
        }
        None
    }
    fn get_child(mut objs:Vec<Box<dyn Hittable>>)->Child{
        if objs.is_empty(){
            return Child::None;
        }
        let aabb = get_aabb(&objs);
        if objs.len() == 1{
            return Child::Node(Box::new(BvhNode{left:Child::Hittable(objs.remove(0)),right:Child::None,aabb}));
        }
        if objs.len() == 2{
            return Child::Node(Box::new(BvhNode{left:Child::Hittable(objs.remove(0)),right:Child::Hittable(objs.remove(0)),aabb}));
        }
        let (left,right) = Self::split_objects(objs);
        let l_node = Self::get_child(left);
        let r_node = Self::get_child(right);
        
        Child::Node(Box::new(BvhNode { left: l_node, right: r_node, aabb }))
    }
    fn split_objects(mut objs:Vec<Box<dyn Hittable>>)->(Vec<Box<dyn Hittable>>,Vec<Box<dyn Hittable>>){
        if objs.is_empty(){
            return (vec![],vec![]);
        }
        let mut bounds:Vec<(Point3,usize)> = objs.iter().enumerate().map(|(i,o)|(o.get_aabb().center(),i)).collect();
        
        let mut sorts:Vec<(usize,Vec<(Point3,usize)>)> = vec![];
        for i in 0..3{
            bounds.sort_by(|a,b|a.0.get(i).partial_cmp(&b.0.get(i)).unwrap_or(Equal));
            sorts.push((i,bounds.clone()));
        }
        sorts.sort_by(|a,b|{
            let a_i = a.0;
            let b_i = b.0;
            let a_len = a.1.last().unwrap().0.get(a_i) - a.1[0].0.get(a_i);
            let b_len = b.1.last().unwrap().0.get(b_i) - b.1[0].0.get(b_i);
            b_len.partial_cmp(&a_len).unwrap_or(Equal)
        });
        
        let sorted_objs= &sorts[0].1;
        let half = sorted_objs.len();

        let mut left = vec![];
        let mut right = vec![];

        for i in 0..half{
             left.push(objs.remove(sorted_objs[i].1));
        }
        for i in half..objs.len(){
            right.push(objs.remove(sorted_objs[i].1));
        }
        (left,right)
        
    }
}


