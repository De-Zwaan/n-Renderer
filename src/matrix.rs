use crate::pos::{Pos2D, Pos3D, Pos4D};
use std::ops;

#[derive(Clone, Copy)]
pub struct Matrix1x2 {
    pub x: Pos2D,
}

#[derive(Clone, Copy)]
pub struct Matrix1x3 {
    pub x: Pos3D,
}

#[derive(Clone, Copy)]
pub struct Matrix1x4 {
    pub x: Pos4D,
}

#[derive(Copy, Clone)]
pub struct Matrix2x2 {
    pub x: Pos2D,
    pub y: Pos2D,
}

#[derive(Clone, Copy)]
pub struct Matrix2x3 {
    pub x: Pos3D,
    pub y: Pos3D,
}

#[derive(Clone, Copy)]
pub struct Matrix2x4 {
    pub x: Pos4D,
    pub y: Pos4D,
}

#[derive(Clone, Copy)]
pub struct Matrix3x2 {
    pub x: Pos2D,
    pub y: Pos2D,
    pub z: Pos2D,
}

#[derive(Copy, Clone)]
pub struct Matrix3x3 {
    pub x: Pos3D,
    pub y: Pos3D,
    pub z: Pos3D,
}

#[derive(Copy, Clone)]
pub struct Matrix3x4 {
    pub x: Pos4D,
    pub y: Pos4D,
    pub z: Pos4D,
}

#[derive(Clone, Copy)]
pub struct Matrix4x2 {
    pub x: Pos2D,
    pub y: Pos2D,
    pub z: Pos2D,
    pub w: Pos2D,
}

#[derive(Copy, Clone)]
pub struct Matrix4x3 {
    pub x: Pos3D,
    pub y: Pos3D,
    pub z: Pos3D,
    pub w: Pos3D,
}

#[derive(Copy, Clone)]
pub struct Matrix4x4 {
    pub x: Pos4D,
    pub y: Pos4D,
    pub z: Pos4D,
    pub w: Pos4D,
}

// Constructors for all matrices
impl Matrix1x2 {
    pub fn new(c: [f64; 2]) -> Self {
        let x: Pos2D = Pos2D { x: c[0], y: c[1] };

        Self { x }
    }
}

impl Matrix1x3 {
    pub fn new(c: [f64; 3]) -> Self {
        let x: Pos3D = Pos3D {
            x: c[0],
            y: c[1],
            z: c[2],
        };

        Self { x }
    }
}

impl Matrix1x4 {
    pub fn new(c: [f64; 4]) -> Self {
        let x: Pos4D = Pos4D {
            x: c[0],
            y: c[1],
            z: c[2],
            w: c[3],
        };

        Self { x }
    }
}

impl Matrix2x2 {
    pub fn new(c: [[f64; 2]; 2]) -> Self {
        let x: Pos2D = Pos2D {
            x: c[0][0],
            y: c[0][1],
        };
        let y: Pos2D = Pos2D {
            x: c[1][0],
            y: c[1][1],
        };

        Self { x, y }
    }
}

impl Matrix2x3 {
    pub fn new(c: [[f64; 3]; 2]) -> Self {
        let x: Pos3D = Pos3D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
        };
        let y: Pos3D = Pos3D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
        };

        Self { x, y }
    }
}

impl Matrix2x4 {
    pub fn new(c: [[f64; 4]; 2]) -> Self {
        let x: Pos4D = Pos4D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
            w: c[0][3],
        };
        let y: Pos4D = Pos4D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
            w: c[1][3],
        };

        Self { x, y }
    }
}

impl Matrix3x2 {
    pub fn new(c: [[f64; 2]; 3]) -> Self {
        let x: Pos2D = Pos2D {
            x: c[0][0],
            y: c[0][1],
        };
        let y: Pos2D = Pos2D {
            x: c[1][0],
            y: c[1][1],
        };
        let z: Pos2D = Pos2D {
            x: c[2][0],
            y: c[2][1],
        };

        Self { x, y, z }
    }
}

impl Matrix3x3 {
    pub fn new(c: [[f64; 3]; 3]) -> Self {
        let x: Pos3D = Pos3D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
        };
        let y: Pos3D = Pos3D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
        };
        let z: Pos3D = Pos3D {
            x: c[2][0],
            y: c[2][1],
            z: c[2][2],
        };

        Self { x, y, z }
    }
}

impl Matrix3x4 {
    pub fn new(c: [[f64; 4]; 3]) -> Self {
        let x: Pos4D = Pos4D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
            w: c[0][3],
        };
        let y: Pos4D = Pos4D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
            w: c[1][3],
        };
        let z: Pos4D = Pos4D {
            x: c[2][0],
            y: c[2][1],
            z: c[2][2],
            w: c[2][3],
        };

        Self { x, y, z }
    }
}

impl Matrix4x2 {
    pub fn new(c: [[f64; 2]; 4]) -> Self {
        let x: Pos2D = Pos2D {
            x: c[0][0],
            y: c[0][1],
        };
        let y: Pos2D = Pos2D {
            x: c[1][0],
            y: c[1][1],
        };
        let z: Pos2D = Pos2D {
            x: c[2][0],
            y: c[2][1],
        };
        let w: Pos2D = Pos2D {
            x: c[3][0],
            y: c[3][1],
        };

        Self { x, y, z, w }
    }
}

impl Matrix4x3 {
    pub fn new(c: [[f64; 3]; 4]) -> Self {
        let x: Pos3D = Pos3D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
        };
        let y: Pos3D = Pos3D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
        };
        let z: Pos3D = Pos3D {
            x: c[2][0],
            y: c[2][1],
            z: c[2][2],
        };
        let w: Pos3D = Pos3D {
            x: c[3][0],
            y: c[3][1],
            z: c[3][2],
        };

        Self { x, y, z, w }
    }
}

impl Matrix4x4 {
    pub fn new(c: [[f64; 4]; 4]) -> Self {
        let x: Pos4D = Pos4D {
            x: c[0][0],
            y: c[0][1],
            z: c[0][2],
            w: c[0][3],
        };
        let y: Pos4D = Pos4D {
            x: c[1][0],
            y: c[1][1],
            z: c[1][2],
            w: c[1][3],
        };
        let z: Pos4D = Pos4D {
            x: c[2][0],
            y: c[2][1],
            z: c[2][2],
            w: c[2][3],
        };
        let w: Pos4D = Pos4D {
            x: c[3][0],
            y: c[3][1],
            z: c[3][2],
            w: c[3][3],
        };

        Self { x, y, z, w }
    }
}

// Addition for all matrices
impl ops::Add for Matrix1x2 {
    type Output = Matrix1x2;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos2D = self.x + rhs.x;

        Self::Output { x }
    }
}

impl ops::Add for Matrix1x3 {
    type Output = Matrix1x3;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos3D = self.x + rhs.x;

        Self::Output { x }
    }
}

impl ops::Add for Matrix1x4 {
    type Output = Matrix1x4;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos4D = self.x + rhs.x;

        Self::Output { x }
    }
}

impl ops::Add for Matrix2x2 {
    type Output = Matrix2x2;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos2D = self.x + rhs.x;
        let y: Pos2D = self.y + rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Add for Matrix2x3 {
    type Output = Matrix2x3;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos3D = self.x + rhs.x;
        let y: Pos3D = self.y + rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Add for Matrix2x4 {
    type Output = Matrix2x4;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos4D = self.x + rhs.x;
        let y: Pos4D = self.y + rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Add for Matrix3x2 {
    type Output = Matrix3x2;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos2D = self.x + rhs.x;
        let y: Pos2D = self.y + rhs.y;
        let z: Pos2D = self.z + rhs.z;

        Self::Output { x, y, z }
    }
}

impl ops::Add for Matrix3x3 {
    type Output = Matrix3x3;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos3D = self.x + rhs.x;
        let y: Pos3D = self.y + rhs.y;
        let z: Pos3D = self.z + rhs.z;

        Self::Output { x, y, z }
    }
}

impl ops::Add for Matrix3x4 {
    type Output = Matrix3x4;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos4D = self.x + rhs.x;
        let y: Pos4D = self.y + rhs.y;
        let z: Pos4D = self.z + rhs.z;

        Self::Output { x, y, z }
    }
}

impl ops::Add for Matrix4x2 {
    type Output = Matrix4x2;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos2D = self.x + rhs.x;
        let y: Pos2D = self.y + rhs.y;
        let z: Pos2D = self.z + rhs.z;
        let w: Pos2D = self.w + rhs.w;

        Self::Output { x, y, z, w }
    }
}

impl ops::Add for Matrix4x3 {
    type Output = Matrix4x3;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos3D = self.x + rhs.x;
        let y: Pos3D = self.y + rhs.y;
        let z: Pos3D = self.z + rhs.z;
        let w: Pos3D = self.w + rhs.w;

        Self::Output { x, y, z, w }
    }
}

impl ops::Add for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, rhs: Self) -> Self::Output {
        let x: Pos4D = self.x + rhs.x;
        let y: Pos4D = self.y + rhs.y;
        let z: Pos4D = self.z + rhs.z;
        let w: Pos4D = self.w + rhs.w;

        Self::Output { x, y, z, w }
    }
}

// Multiplication for 1x2 matrix
impl ops::Mul<f64> for Matrix1x2 {
    type Output = Matrix1x2;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos2D = self.x * rhs;

        Self::Output { x }
    }
}

impl ops::Mul<Pos2D> for Matrix1x2 {
    type Output = f64;

    fn mul(self, rhs: Pos2D) -> Self::Output {
        self.x.x * rhs.x + self.x.y * rhs.y
    }
}

impl ops::Mul<Matrix2x2> for Matrix1x2 {
    type Output = Matrix1x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix2x3> for Matrix1x2 {
    type Output = Matrix1x3;

    fn mul(self, rhs: Matrix2x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix2x4> for Matrix1x2 {
    type Output = Matrix1x4;

    fn mul(self, rhs: Matrix2x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w,
        };

        Self::Output { x }
    }
}

// Multiplication for 1x3 matrix
impl ops::Mul<f64> for Matrix1x3 {
    type Output = Matrix1x3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos3D = self.x * rhs;

        Self::Output { x }
    }
}

impl ops::Mul<Pos3D> for Matrix1x3 {
    type Output = f64;

    fn mul(self, rhs: Pos3D) -> Self::Output {
        self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z
    }
}

impl ops::Mul<Matrix3x2> for Matrix1x3 {
    type Output = Matrix1x2;

    fn mul(self, rhs: Matrix3x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix3x3> for Matrix1x3 {
    type Output = Matrix1x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix3x4> for Matrix1x3 {
    type Output = Matrix1x4;

    fn mul(self, rhs: Matrix3x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z * self.x.z * rhs.z.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w,
        };

        Self::Output { x }
    }
}

// Multiplication for 1x4 matrix
impl ops::Mul<f64> for Matrix1x4 {
    type Output = Matrix1x4;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos4D = self.x * rhs;

        Self::Output { x }
    }
}

impl ops::Mul<Pos4D> for Matrix1x4 {
    type Output = f64;

    fn mul(self, rhs: Pos4D) -> Self::Output {
        self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w
    }
}

impl ops::Mul<Matrix4x2> for Matrix1x4 {
    type Output = Matrix1x2;

    fn mul(self, rhs: Matrix4x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix4x3> for Matrix1x4 {
    type Output = Matrix1x3;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
        };

        Self::Output { x }
    }
}

impl ops::Mul<Matrix4x4> for Matrix1x4 {
    type Output = Matrix1x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
        };

        Self::Output { x }
    }
}

// Multiplication for 2x1 matrix (Pos2D)
impl ops::Mul<Matrix1x2> for Pos2D {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix1x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix1x3> for Pos2D {
    type Output = Matrix2x3;

    fn mul(self, rhs: Matrix1x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix1x4> for Pos2D {
    type Output = Matrix2x4;

    fn mul(self, rhs: Matrix1x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
            w: self.x * rhs.x.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
            w: self.y * rhs.x.w,
        };

        Self::Output { x, y }
    }
}

// Multiplication for 2x2 matrix
impl ops::Mul<f64> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos2D = self.x * rhs;
        let y: Pos2D = self.y * rhs;

        Self::Output { x, y }
    }
}

impl ops::Mul<Pos2D> for Matrix2x2 {
    type Output = Pos2D;

    fn mul(self, rhs: Pos2D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix2x3> for Matrix2x2 {
    type Output = Matrix2x3;

    fn mul(self, rhs: Matrix2x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix2x4> for Matrix2x2 {
    type Output = Matrix2x4;

    fn mul(self, rhs: Matrix2x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w,
        };

        Self::Output { x, y }
    }
}

// Multiplication for 2x3 matrix
impl ops::Mul<f64> for Matrix2x3 {
    type Output = Matrix2x3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos3D = self.x * rhs;
        let y: Pos3D = self.y * rhs;

        Self::Output { x, y }
    }
}

impl ops::Mul<Pos3D> for Matrix2x3 {
    type Output = Pos2D;

    fn mul(self, rhs: Pos3D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z;

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix3x2> for Matrix2x3 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix3x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix3x3> for Matrix2x3 {
    type Output = Matrix2x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix3x4> for Matrix2x3 {
    type Output = Matrix2x4;

    fn mul(self, rhs: Matrix3x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z * self.x.z * rhs.z.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z * self.y.z * rhs.z.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w,
        };

        Self::Output { x, y }
    }
}

// Multiplication for 2x4 matrix
impl ops::Mul<f64> for Matrix2x4 {
    type Output = Matrix2x4;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos4D = self.x * rhs;
        let y: Pos4D = self.y * rhs;

        Self::Output { x, y }
    }
}

impl ops::Mul<Pos4D> for Matrix2x4 {
    type Output = Pos2D;

    fn mul(self, rhs: Pos4D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w;

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix4x2> for Matrix2x4 {
    type Output = Matrix2x2;

    fn mul(self, rhs: Matrix4x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix4x3> for Matrix2x4 {
    type Output = Matrix2x3;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
        };

        Self::Output { x, y }
    }
}

impl ops::Mul<Matrix4x4> for Matrix2x4 {
    type Output = Matrix2x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
        };

        Self::Output { x, y }
    }
}

// Multiplication for 3x1 matrix (Pos3D)
impl ops::Mul<Matrix1x2> for Pos3D {
    type Output = Matrix3x2;

    fn mul(self, rhs: Matrix1x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix1x3> for Pos3D {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix1x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
            z: self.z * rhs.x.z,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix1x4> for Pos3D {
    type Output = Matrix3x4;

    fn mul(self, rhs: Matrix1x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
            w: self.x * rhs.x.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
            w: self.y * rhs.x.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
            z: self.z * rhs.x.z,
            w: self.z * rhs.x.w,
        };

        Self::Output { x, y, z }
    }
}

// Multiplication for 3x2 matrix
impl ops::Mul<f64> for Matrix3x2 {
    type Output = Matrix3x2;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos2D = self.x * rhs;
        let y: Pos2D = self.y * rhs;
        let z: Pos2D = self.z * rhs;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Pos2D> for Matrix3x2 {
    type Output = Pos3D;

    fn mul(self, rhs: Pos2D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix2x2> for Matrix3x2 {
    type Output = Matrix3x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix2x3> for Matrix3x2 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix2x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix2x4> for Matrix3x2 {
    type Output = Matrix3x4;

    fn mul(self, rhs: Matrix2x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w,
        };

        Self::Output { x, y, z }
    }
}

// Multiplication for 3x3 matrix
impl ops::Mul<f64> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos3D = self.x * rhs;
        let y: Pos3D = self.y * rhs;
        let z: Pos3D = self.z * rhs;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Pos3D> for Matrix3x3 {
    type Output = Pos3D;

    fn mul(self, rhs: Pos3D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix3x2> for Matrix3x3 {
    type Output = Matrix3x2;

    fn mul(self, rhs: Matrix3x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix3x4> for Matrix3x3 {
    type Output = Matrix3x4;

    fn mul(self, rhs: Matrix3x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z * self.x.z * rhs.z.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z * self.y.z * rhs.z.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z * self.z.z * rhs.z.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w,
        };

        Self::Output { x, y, z }
    }
}

// Multiplication for 3x4 matrix
impl ops::Mul<f64> for Matrix3x4 {
    type Output = Matrix3x4;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos4D = self.x * rhs;
        let y: Pos4D = self.y * rhs;
        let z: Pos4D = self.z * rhs;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Pos4D> for Matrix3x4 {
    type Output = Pos3D;

    fn mul(self, rhs: Pos4D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z + self.z.w * rhs.w;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix4x2> for Matrix3x4 {
    type Output = Matrix3x2;

    fn mul(self, rhs: Matrix4x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix4x3> for Matrix3x4 {
    type Output = Matrix3x3;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
        };

        Self::Output { x, y, z }
    }
}

impl ops::Mul<Matrix4x4> for Matrix3x4 {
    type Output = Matrix3x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,
        };

        Self::Output { x, y, z }
    }
}

// Multiplication for 4x1 matrix (Pos4D)
impl ops::Mul<Matrix1x2> for Pos4D {
    type Output = Matrix4x2;

    fn mul(self, rhs: Matrix1x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
        };
        let w: Pos2D = Pos2D {
            x: self.w * rhs.x.x,
            y: self.w * rhs.x.y,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix1x3> for Pos4D {
    type Output = Matrix4x3;

    fn mul(self, rhs: Matrix1x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
            z: self.z * rhs.x.z,
        };
        let w: Pos3D = Pos3D {
            x: self.w * rhs.x.x,
            y: self.w * rhs.x.y,
            z: self.w * rhs.x.z,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix1x4> for Pos4D {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix1x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x * rhs.x.x,
            y: self.x * rhs.x.y,
            z: self.x * rhs.x.z,
            w: self.x * rhs.x.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y * rhs.x.x,
            y: self.y * rhs.x.y,
            z: self.y * rhs.x.z,
            w: self.y * rhs.x.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z * rhs.x.x,
            y: self.z * rhs.x.y,
            z: self.z * rhs.x.z,
            w: self.z * rhs.x.w,
        };
        let w: Pos4D = Pos4D {
            x: self.w * rhs.x.x,
            y: self.w * rhs.x.y,
            z: self.w * rhs.x.z,
            w: self.w * rhs.x.w,
        };

        Self::Output { x, y, z, w }
    }
}

// Multiplication for 4x2 matrix
impl ops::Mul<f64> for Matrix4x2 {
    type Output = Matrix4x2;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos2D = self.x * rhs;
        let y: Pos2D = self.y * rhs;
        let z: Pos2D = self.z * rhs;
        let w: Pos2D = self.w * rhs;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Pos2D> for Matrix4x2 {
    type Output = Pos4D;

    fn mul(self, rhs: Pos2D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y;
        let w: f64 = self.w.x * rhs.x + self.w.y * rhs.y;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix2x2> for Matrix4x2 {
    type Output = Matrix4x2;

    fn mul(self, rhs: Matrix2x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
        };
        let w: Pos2D = Pos2D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix2x3> for Matrix4x2 {
    type Output = Matrix4x3;

    fn mul(self, rhs: Matrix2x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z,
        };
        let w: Pos3D = Pos3D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix2x4> for Matrix4x2 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix2x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w,
        };
        let w: Pos4D = Pos4D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z,
            w: self.w.x * rhs.x.w + self.w.y * rhs.y.w,
        };

        Self::Output { x, y, z, w }
    }
}

// Multiplication for 4x3 matrix
impl ops::Mul<f64> for Matrix4x3 {
    type Output = Matrix4x3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos3D = self.x * rhs;
        let y: Pos3D = self.y * rhs;
        let z: Pos3D = self.z * rhs;
        let w: Pos3D = self.w * rhs;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Pos3D> for Matrix4x3 {
    type Output = Pos4D;

    fn mul(self, rhs: Pos3D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z;
        let w: f64 = self.w.x * rhs.x + self.w.y * rhs.y + self.w.z * rhs.z;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix3x2> for Matrix4x3 {
    type Output = Matrix4x2;

    fn mul(self, rhs: Matrix3x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
        };
        let w: Pos2D = Pos2D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix3x3> for Matrix4x3 {
    type Output = Matrix4x3;

    fn mul(self, rhs: Matrix3x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z,
        };
        let w: Pos3D = Pos3D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix3x4> for Matrix4x3 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix3x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z * self.x.z * rhs.z.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z * self.y.z * rhs.z.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z * self.z.z * rhs.z.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w,
        };
        let w: Pos4D = Pos4D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z * self.w.z * rhs.z.z,
            w: self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w,
        };

        Self::Output { x, y, z, w }
    }
}

// Multiplication for 4x4 matrix
impl ops::Mul<f64> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: Pos4D = self.x * rhs;
        let y: Pos4D = self.y * rhs;
        let z: Pos4D = self.z * rhs;
        let w: Pos4D = self.w * rhs;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Pos4D> for Matrix4x4 {
    type Output = Pos4D;

    fn mul(self, rhs: Pos4D) -> Self::Output {
        let x: f64 = self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w;
        let y: f64 = self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w;
        let z: f64 = self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z + self.z.w * rhs.w;
        let w: f64 = self.w.x * rhs.x + self.w.y * rhs.y + self.w.z * rhs.z + self.w.w * rhs.w;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix4x2> for Matrix4x4 {
    type Output = Matrix4x2;

    fn mul(self, rhs: Matrix4x2) -> Self::Output {
        let x: Pos2D = Pos2D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
        };
        let y: Pos2D = Pos2D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
        };
        let z: Pos2D = Pos2D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
        };
        let w: Pos2D = Pos2D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix4x3> for Matrix4x4 {
    type Output = Matrix4x3;

    fn mul(self, rhs: Matrix4x3) -> Self::Output {
        let x: Pos3D = Pos3D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
        };
        let y: Pos3D = Pos3D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
        };
        let z: Pos3D = Pos3D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
        };
        let w: Pos3D = Pos3D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
        };

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let x: Pos4D = Pos4D {
            x: self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
            y: self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
            z: self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
            w: self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
        };
        let y: Pos4D = Pos4D {
            x: self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
            y: self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
            z: self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
            w: self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
        };
        let z: Pos4D = Pos4D {
            x: self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
            y: self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
            z: self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
            w: self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,
        };
        let w: Pos4D = Pos4D {
            x: self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
            y: self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
            z: self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
            w: self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w + self.w.w * rhs.w.w,
        };

        Self::Output { x, y, z, w }
    }
}
