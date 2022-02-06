use image::{RgbImage};
use ndarray::Array3;

pub fn array_to_image(arr:Array3<u8>)->RgbImage{
    arr.is_standard_layout();
    
    let (height,width,_) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw).expect("array size error")
}