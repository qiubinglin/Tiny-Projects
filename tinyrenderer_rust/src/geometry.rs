use std::ops::{Add, Sub, Mul, Div, Index};
use std::f64;

/********************************************************************************/

#[derive(Copy, Clone)]
pub struct Vec2D {
    arr: [f64; 2],
}

impl Vec2D {
    #[allow(dead_code)]
    fn norm2(self) -> f64 {
        self*self
    }

    #[allow(dead_code)]
    fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }
}

impl Index<usize> for Vec2D {
    type Output = f64;

    fn index(&self, rhs: usize) -> &f64 {
        &self.arr[rhs]
    }
}

impl Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Vec2D) -> Vec2D {
        Vec2D { arr: [ self.arr[0]+rhs.arr[0], self.arr[1]+rhs.arr[1] ] }
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Vec2D) -> Vec2D {
        Vec2D { arr: [ self.arr[0]-rhs.arr[0], self.arr[1]-rhs.arr[1] ] }
    }
}

impl Mul<Vec2D> for Vec2D {
    type Output = f64;

    fn mul(self, rhs: Vec2D) -> f64 {
        self.arr[0]*rhs.arr[0] + self.arr[1]*rhs.arr[1]
    }
}

impl Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f64) -> Vec2D {
        Vec2D { arr: [self.arr[0]*rhs, self.arr[1]*rhs] }
    }
}

impl Mul<Vec2D> for f64 {
    type Output = Vec2D;

    fn mul(self, rhs: Vec2D) -> Vec2D {
        Vec2D { arr: [self*rhs.arr[0], self*rhs.arr[1]] }
    }
}

impl Div<f64> for Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f64) -> Vec2D {
        Vec2D { arr: [self.arr[0]/rhs, self.arr[1]/rhs] }
    }
}

/********************************************************************************/
/********************************************************************************/

#[derive(Copy, Clone)]
pub struct Vec3D {
    arr: [f64; 3],
}

impl Vec3D {
    #[allow(dead_code)]
    fn norm2(self) -> f64 {
        self*self
    }

    #[allow(dead_code)]
    fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }
}

impl Index<usize> for Vec3D {
    type Output = f64;

    fn index(&self, rhs: usize) -> &f64 {
        &self.arr[rhs]
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [ self.arr[0]+rhs.arr[0], self.arr[1]+rhs.arr[1], self.arr[2]+rhs.arr[2] ] }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [ self.arr[0]-rhs.arr[0], self.arr[1]-rhs.arr[1], self.arr[2]-rhs.arr[2] ] }
    }
}

impl Mul<Vec3D> for Vec3D {
    type Output = f64;

    fn mul(self, rhs: Vec3D) -> f64 {
        self.arr[0]*rhs.arr[0] + self.arr[1]*rhs.arr[1] + self.arr[2]*rhs.arr[2]
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    fn mul(self, rhs: f64) -> Vec3D {
        Vec3D { arr: [self.arr[0]*rhs, self.arr[1]*rhs, self.arr[2]*rhs] }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, rhs: Vec3D) -> Vec3D {
        Vec3D { arr: [self*rhs.arr[0], self*rhs.arr[1], self*rhs.arr[2]] }
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, rhs: f64) -> Vec3D {
        Vec3D { arr: [self.arr[0]/rhs, self.arr[1]/rhs, self.arr[2]/rhs] }
    }
}

/********************************************************************************/
/********************************************************************************/

#[derive(Copy, Clone)]
pub struct Vec4D {
    arr: [f64; 4],
}

impl Vec4D {
    #[allow(dead_code)]
    fn norm2(self) -> f64 {
        self*self
    }

    #[allow(dead_code)]
    fn norm(self) -> f64 {
        f64::sqrt(self.norm2())
    }
}

impl Index<usize> for Vec4D {
    type Output = f64;

    fn index(&self, rhs: usize) -> &f64 {
        &self.arr[rhs]
    }
}

impl Add for Vec4D {
    type Output = Vec4D;

    fn add(self, rhs: Vec4D) -> Vec4D {
        Vec4D { arr: [ self.arr[0]+rhs.arr[0], self.arr[1]+rhs.arr[1], self.arr[2]+rhs.arr[2], self.arr[3]+rhs.arr[3] ] }
    }
}

impl Sub for Vec4D {
    type Output = Vec4D;

    fn sub(self, rhs: Vec4D) -> Vec4D {
        Vec4D { arr: [ self.arr[0]-rhs.arr[0], self.arr[1]-rhs.arr[1], self.arr[2]-rhs.arr[2], self.arr[3]-rhs.arr[3] ] }
    }
}

impl Mul<Vec4D> for Vec4D {
    type Output = f64;

    fn mul(self, rhs: Vec4D) -> f64 {
        self.arr[0]*rhs.arr[0] + self.arr[1]*rhs.arr[1] + self.arr[2]*rhs.arr[2] + self.arr[3]*rhs.arr[3]
    }
}

impl Mul<f64> for Vec4D {
    type Output = Vec4D;

    fn mul(self, rhs: f64) -> Vec4D {
        Vec4D { arr: [self.arr[0]*rhs, self.arr[1]*rhs, self.arr[2]*rhs, self.arr[3]*rhs] }
    }
}

impl Mul<Vec4D> for f64 {
    type Output = Vec4D;

    fn mul(self, rhs: Vec4D) -> Vec4D {
        Vec4D { arr: [self*rhs.arr[0], self*rhs.arr[1], self*rhs.arr[2], self*rhs.arr[3]] }
    }
}

impl Div<f64> for Vec4D {
    type Output = Vec4D;

    fn div(self, rhs: f64) -> Vec4D {
        Vec4D { arr: [self.arr[0]/rhs, self.arr[1]/rhs, self.arr[2]/rhs, self.arr[3]/rhs] }
    }
}

/********************************************************************************/