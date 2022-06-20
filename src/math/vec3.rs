use rand::{distributions::Uniform, prelude::Distribution};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {

  pub fn forward(self,dir:Vec3,bias:f32)->Vec3{
    self+dir.to_unit()*bias
  }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn to_unit(self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn get(&self,i:usize)->f32{
      if i==0{
        return self.x
      }
      else if i==1{
        return self.y
      }
      else if i==2{
        return self.z
      }
      panic!("vector index out of range")
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn from_array(array:[f32;3])->Vec3{
        Vec3::new(array[0],array[1],array[2])
    }
    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let rand_range = Uniform::new(-1.0f32, 1.0f32);
        Vec3::new(
            rand_range.sample(&mut rng),
            rand_range.sample(&mut rng),
            rand_range.sample(&mut rng),
        )
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let rand = Vec3::random();
            if rand.length_squared() < 1.0 {
                return rand;
            }
        }
    }
    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }
    pub fn refract(out_n: f32, in_n: f32, in_dir: Vec3, normal: Vec3) -> Option<Vec3> {
        let cos = -in_dir.dot(normal).min(1.0);
        let sin = (1.0-cos*cos).sqrt();

        let plane = (in_dir + cos * normal).to_unit();
        let p = plane * out_n / in_n *sin;
        //println!("{}",p.length());
        let temp = p.length().powf(2.0);
        if temp>1.0{     
          return None//return Vec3::reflect(in_dir, normal);
        }
        let normal_through = normal * -(1.0 -temp).sqrt();
        Some(normal_through + p)
    }

    pub fn cross(&self,v:Vec3)->Vec3{
      Vec3::new(self.y*v.z - self.z*v.y,self.z*v.x - self.x*v.z,self.x*v.y - self.y*v.x)
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
          x: self.x $op_symbol other as f32,
          y: self.y $op_symbol other as f32,
          z: self.z $op_symbol other as f32
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

macro_rules! impl_op_assign {
  ($VectorType:ident $OperationAssign:ident $op_fn:ident $op_symbol:tt) => {
    impl<'a> $OperationAssign<&'a $VectorType> for $VectorType {
      fn $op_fn(&mut self, other: &'a $VectorType) {
        *self = $VectorType {
          x: self.x $op_symbol other.x,
          y: self.y $op_symbol other.y,
          z: self.z $op_symbol other.z,
        };
      }
    }

    impl $OperationAssign for $VectorType {
      #[inline]
      fn $op_fn(&mut self, other: $VectorType) {
        *self = *self $op_symbol &other
      }
    }
  };
}

impl_binary_operations!(Vec3 Add add +);
impl_op_assign!(Vec3 AddAssign add_assign +);
impl_number_operations!(Vec3 Add add + f32);
impl_number_operations!(Vec3 Add add + i32);
impl_number_operations!(Vec3 Add add + u32);
impl_number_operations!(Vec3 Add add + usize);

impl_binary_operations!(Vec3 Sub sub -);
impl_op_assign!(Vec3 SubAssign  sub_assign -);
impl_number_operations!(Vec3 Sub sub - f32);
impl_number_operations!(Vec3 Sub sub - i32);
impl_number_operations!(Vec3 Sub sub - u32);
impl_number_operations!(Vec3 Sub sub - usize);

impl_binary_operations!(Vec3 Mul mul *);
impl_op_assign!(Vec3 MulAssign  mul_assign *);
impl_number_operations!(Vec3 Mul mul * f32);
impl_number_operations!(Vec3 Mul mul * i32);
impl_number_operations!(Vec3 Mul mul * u32);
impl_number_operations!(Vec3 Mul mul * usize);

impl_binary_operations!(Vec3 Div div /);
impl_op_assign!(Vec3 DivAssign  div_assign /);
impl_number_operations!(Vec3 Div div / f32);
impl_number_operations!(Vec3 Div div / i32);
impl_number_operations!(Vec3 Div div / u32);
impl_number_operations!(Vec3 Div div / usize);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color{
  pub fn from_rgb(r:u8,g:u8,b:u8)->Color{
    Color{x:r as f32/255.0,y:g as f32/255.0,z:b as f32/255.0}
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        let c: f32 = 3.0;
        assert_eq!(&a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + &b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(&a + b, Vec3::new(3.0, 5.0, 7.0));
        assert_eq!(a + b, Vec3::new(3.0, 5.0, 7.0));
        println!("{:?},{:?},{:?}", a, b, c);

        assert_eq!(&a + c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(&a + c, Vec3::new(3.0, 4.0, 5.0));
        assert_eq!(a + c, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn multiply() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(&a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * &b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(&a * b, Vec3::new(0.0, 4.0, 10.0));
        assert_eq!(a * b, Vec3::new(0.0, 4.0, 10.0));
    }

    fn num_test() {}
}
