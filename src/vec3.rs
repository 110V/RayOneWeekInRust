use num::{ToPrimitive};
use std::ops::{Add, Div, Mul, Sub};
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn dot(self,other:Vec3)->f32{
      self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn to_unit(self)->Vec3{
      let length = self.length();
      Vec3::new(self.x/length, self.y/length, self.z/length)
    }
    pub fn length_squared(&self) -> f32 {
      self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self)->f32 {
      self.length_squared().sqrt()
    }

    pub fn to_array(&self)->[f32;3] {
      [self.x,self.y,self.z]
    }

    pub fn sum(&self)->f32 {
      self.x+self.y+self.z
    }
}

macro_rules! impl_binary_operations {
    ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
      impl $Operation<&$VectorType> for &'_ $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: &$VectorType) -> $VectorType {
          $VectorType {
            x: self.x $op_symbol other.x,
            y: self.y $op_symbol other.y,
            z: self.z $op_symbol other.z,
          }
        }
      }

      impl $Operation<&'_ $VectorType> for $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: &'_ $VectorType) -> $VectorType {
            &self $op_symbol other
        }
      }

      impl<'a> $Operation<$VectorType> for &'a $VectorType {
        type Output = $VectorType;

        fn $op_fn(self, other: $VectorType) -> $VectorType {
            self $op_symbol &other
        }
      }

      impl $Operation<$VectorType> for $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: $VectorType) -> $VectorType {
            &self $op_symbol &other
        }
      }

    }
}

macro_rules! impl_number_operations {
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt $num_type:ident) => {
    impl<'a,'b> $Operation<$num_type> for &'b $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other:$num_type) -> $VectorType {
        $VectorType{
          x: self.x $op_symbol other.to_f32().unwrap(),
          y: self.y $op_symbol other.to_f32().unwrap(),
          z: self.z $op_symbol other.to_f32().unwrap()
        }
      }
    }
    impl $Operation<$num_type> for $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other:$num_type) -> $VectorType {
        &self $op_symbol other
      }
    }
    impl $Operation<$VectorType> for $num_type {
      type Output = $VectorType;
      fn $op_fn(self, other:$VectorType) -> $VectorType {
        other $op_symbol self
      }
    }

    impl $Operation<&'_ $VectorType> for $num_type {
      type Output = $VectorType;
      fn $op_fn(self, other:&'_ $VectorType) -> $VectorType {
        other $op_symbol self
      }
    }
  }
}


impl_binary_operations!(Vec3 Add add +);
impl_number_operations!(Vec3 Add add + f32);
impl_number_operations!(Vec3 Add add + i32);
impl_number_operations!(Vec3 Add add + u32);
impl_number_operations!(Vec3 Add add + usize);

impl_binary_operations!(Vec3 Sub sub -);
impl_number_operations!(Vec3 Sub sub - f32);
impl_number_operations!(Vec3 Sub sub - i32);
impl_number_operations!(Vec3 Sub sub - u32);
impl_number_operations!(Vec3 Sub sub - usize);

impl_binary_operations!(Vec3 Mul mul *);
impl_number_operations!(Vec3 Mul mul * f32);
impl_number_operations!(Vec3 Mul mul * i32);
impl_number_operations!(Vec3 Mul mul * u32);
impl_number_operations!(Vec3 Mul mul * usize);

impl_binary_operations!(Vec3 Div div /);
impl_number_operations!(Vec3 Div div / f32);
impl_number_operations!(Vec3 Div div / i32);
impl_number_operations!(Vec3 Div div / u32);
impl_number_operations!(Vec3 Div div / usize);


pub type Point3 = Vec3;
pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        let c:f32 = 3.0;
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));
        println!("{:?},{:?},{:?}",a,b,c);

        assert_eq!(&a+c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(&a+c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(a+c, Vec3::new(3.0, 4.0, 5.0));
    }

    fn num_test(){
      
    }
}
