use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec2d<T: Copy> {
    pub x: T,
    pub y: T,
}

impl Vec2d<usize> {
    pub fn len(&self) -> f64 {
        (((self.x * self.x) + (self.y * self.y)) as f64).sqrt()
    }
}
impl Vec2d<isize> {
    pub fn len(&self) -> f64 {
        (((self.x * self.x) + (self.y * self.y)) as f64).sqrt()
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec2d<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec2d<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec2d<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Vec2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2d<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Vec2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
