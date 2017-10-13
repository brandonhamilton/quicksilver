use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::cmp::{Eq, PartialEq};
use std::fmt;

pub const FLOAT_LIMIT: f32 = 0.000001f32;

#[derive(Debug, Copy, Clone)]
///A 2D vector with an arbitrary numeric type
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
    ///The zero vector
    pub fn zero() -> Vector {
        Vector { x: 0f32, y: 0f32 }
    }

    ///A vector with x = 1f32, y = 0f32
    pub fn x() -> Vector {
        Vector { x: 1f32, y: 0f32 }
    }

    ///A vector with x = 0f32, y = 1f32
    pub fn y() -> Vector {
        Vector { x: 0f32, y: 1f32 }
    }

    ///A vector with x = 1f32, y = 1f32
    pub fn one() -> Vector {
        Vector { x: 1f32, y : 1f32 }
    }
   
    pub fn new(x: f32, y: f32) -> Vector {
       Vector { x: x, y: y }
    }

    pub fn newi(x: i32, y: i32) -> Vector {
        Vector::new(x as f32, y as f32)
    }

    ///Get the squared length of the vector (faster than getting the length)
    pub fn len2(self) -> f32 {
       self.x * self.x + self.y * self.y
    }

    ///Get the length of the vector
    pub fn len(self) -> f32 {
       self.len2().sqrt()
    }

    ///Clamp a vector somewhere between a minimum and a maximum
    pub fn clamp(self, min_bound: Vector, max_bound: Vector) -> Vector {
       Vector::new(max_bound.x.min(min_bound.x.max(self.x)),
           max_bound.y.min(min_bound.y.max(self.y)))
    }

    ///Get the cross product of a vector
    pub fn cross(self, other: Vector) -> f32 {
       self.x * other.y - self.y * other.x
    }

    ///Get the dot product of a vector
    pub fn dot(self, other: Vector) -> f32 {
       self.x * other.x + self.y * other.y
    }

    ///Normalize the vector's length from [0, 1]
    pub fn normalize(self) -> Vector {
       self / self.len()
    }

    ///Get only the X component of the Vector, represented as a vector
    pub fn x_comp(self) -> Vector {
       Vector::new(self.x, 0f32)
    }

    ///Get only the Y component of the Vector, represented as a vector
    pub fn y_comp(self) -> Vector {
       Vector::new(0f32, self.y)
    }

    ///Get the vector equal to Vector(1 / x, 1 / y)
    pub fn recip(self) -> Vector {
       Vector::new(self.x.recip(), self.y.recip())
    }

    ///Multiply the components in the matching places
    pub fn times(self, other: Vector) -> Vector {
       Vector::new(self.x * other.x, self.y * other.y)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) -> () {
        *self = *self + rhs;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        self + (-rhs)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) -> () {
        *self = *self - rhs;
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) -> () {
        *self = *self / rhs;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) -> () {
        *self = *self * rhs;
    }
}

impl Div<i32> for Vector {
    type Output = Vector;

    fn div(self, rhs: i32) -> Vector {
        Vector::new(self.x / rhs as f32, self.y / rhs as f32)
    }
}

impl DivAssign<i32> for Vector {
    fn div_assign(&mut self, rhs: i32) -> () {
        *self = *self / rhs;
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Vector {
        Vector::new(self.x * rhs as f32, self.y * rhs as f32)
    }
}

impl MulAssign<i32> for Vector {
    fn mul_assign(&mut self, rhs: i32) -> () {
        *self = *self * rhs;
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() < FLOAT_LIMIT && (self.y - other.y).abs() < FLOAT_LIMIT
    }
}

impl Eq for Vector {}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let a = Vector::newi(5, 10);
        let b = Vector::newi(1, -2);
        assert!((a + b).x == 6f32);
        assert!((a - b).y == 12f32);
    }

    #[test]
    fn equality() {
        assert_eq!(Vector::newi(5, 5), Vector::newi(5, 5));
        assert_ne!(Vector::newi(0, 5), Vector::newi(5, 5));
    }

    #[test]
    fn inverse() {
        let vec = Vector::newi(3, 5);
        let inverse = vec.recip();
        assert!((inverse.x - vec.x.recip()).abs() < FLOAT_LIMIT &&
                (inverse.y - vec.y.recip()).abs() < FLOAT_LIMIT);
    }

    #[test]
    fn length() {
        let vec = Vector::x() * 5;
        assert!((vec.len2() - 25f32).abs() < FLOAT_LIMIT);
        assert!((vec.len() - 5f32).abs() < FLOAT_LIMIT);
    }

    #[test]
    fn scale() {
        let vec = Vector::newi(1, 1);
        let doubled = Vector::newi(2, 2);
        assert_eq!(vec * 2, doubled);
        let halved = Vector::new(0.5, 0.5);
        assert_eq!(vec / 2, halved);
    }

    #[test]
    fn clamp() {
        let min = Vector::newi(-10, -2);
        let max = Vector::newi(5, 6);
        let vec = Vector::newi(-11, 3);
        let clamped = vec.clamp(min, max);
        let expected = Vector::newi(-10, 3);
        assert_eq!(clamped, expected);
    }

    #[test]
    fn dot() {
        assert!((Vector::newi(6, 5).dot(Vector::newi(2, -8)) - 28f32) <= FLOAT_LIMIT);
    }

    #[test]
    fn times() {
        let vec = Vector::newi(3, -2);
        let two = Vector::one() * 2;
        assert_eq!(vec.times(two), vec * 2);
    }
}
