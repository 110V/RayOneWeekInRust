use std::ops::Add;
use num::{Float,Integer};
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
impl_binary_operations!(Vec3 Add add +);


// impl<'a,T:Float+Integer> $Operation<T> for &'a $VectorType {
//   type Output = $VectorType;

//   fn $op_fn(self, other: T) -> $VectorType {
//     $VectorType {
//       x: self.x $op_symbol other,
//       y: self.y $op_symbol other,
//       z: self.z $op_symbol other
//     }
//   }

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));
    }
  }