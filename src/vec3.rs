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
}

macro_rules! impl_binary_operations {
    ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {
      impl<'a, 'b> $Operation<&'a $VectorType> for &'b $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
          $VectorType {
            x: self.x $op_symbol other.x,
            y: self.y $op_symbol other.y,
            z: self.z $op_symbol other.z,
          }
        }
      }

      impl<'a> $Operation<&'a $VectorType> for $VectorType {
        type Output = $VectorType;
        fn $op_fn(self, other: &'a $VectorType) -> $VectorType {
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
  ($VectorType:ident $Operation:ident $op_fn:ident $op_symbol:tt) => {

    impl<'a,'b,T:ToPrimitive> $Operation<T> for &'b $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other:T) -> $VectorType {
        $VectorType{
          x: self.x $op_symbol other.to_f32().unwrap(),
          y: self.y $op_symbol other.to_f32().unwrap(),
          z: self.z $op_symbol other.to_f32().unwrap()
        }
      }
    }
    impl<T:ToPrimitive> $Operation<T> for $VectorType {
      type Output = $VectorType;
      fn $op_fn(self, other:T) -> $VectorType {
        &self $op_symbol other
      }
    }
  }
}


impl_binary_operations!(Vec3 Add add +);
impl_number_operations!(Vec3 Add add +);

impl_binary_operations!(Vec3 Sub sub -);
impl_number_operations!(Vec3 Sub sub -);

impl_binary_operations!(Vec3 Mul mul *);
impl_number_operations!(Vec3 Mul mul *);

impl_binary_operations!(Vec3 Div div /);
impl_number_operations!(Vec3 Div div /);



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        let c:u32 = 3;
        let d:i32 = -3;
        let e:f32 = 3.0;
        let f:usize = 10;
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));
        println!("{:?},{:?},{:?},{:?},{:?},{:?}",a,b,c,d,e,f);

        assert_eq!(&a+c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(&a+c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(a+c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(&a+d, Vec3::new(-3.0, -2.0, -1.0));
        assert_eq!(&a+e, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(&a+f, Vec3::new(10.0, 11.0, 12.0));
    }

    fn num_test(){
      
    }
}
