use std::ops;

use winit::dpi::PhysicalSize;

use crate::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};

#[derive(Clone, Copy, Debug)]
pub enum RotationPlane {
    XY,
    XZ,
    XW,
    YX,
    YZ,
    YW,
    ZX,
    ZY,
    ZW,
    WX,
    WY,
    WZ,
}

impl RotationPlane {
    pub fn get_rot_mat_4d(plane: RotationPlane, angle: f64) -> Matrix4x4 {
        let cos: f64 = angle.cos();
        let sin: f64 = angle.sin();

        use RotationPlane::*;

        match plane {
            XY => Matrix4x4::new([
                [cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            XZ => Matrix4x4::new([
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            XW => Matrix4x4::new([
                [cos, 0.0, 0.0, sin],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-sin, 0.0, 0.0, cos],
            ]),
            YX => Matrix4x4::new([
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            YZ => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            YW => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, 0.0, sin],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, -sin, 0.0, cos],
            ]),
            ZX => Matrix4x4::new([
                [cos, 0.0, -sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            ZY => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]),
            ZW => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, cos, sin],
                [0.0, 0.0, -sin, cos],
            ]),
            WX => Matrix4x4::new([
                [cos, 0.0, 0.0, -sin],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [sin, 0.0, 0.0, cos],
            ]),
            WY => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, 0.0, -sin],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, sin, 0.0, cos],
            ]),
            WZ => Matrix4x4::new([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, cos, -sin],
                [0.0, 0.0, sin, cos],
            ]),
        }
    }

    pub fn _get_rot_mat_3d(plane: RotationPlane, angle: f64) -> Matrix3x3 {
        let cos: f64 = angle.cos();
        let sin: f64 = angle.sin();

        use RotationPlane::*;

        match plane {
            XY => Matrix3x3::new([[cos, sin, 0.0], [-sin, cos, 0.0], [0.0, 0.0, 1.0]]),
            XZ => Matrix3x3::new([[cos, 0.0, sin], [0.0, 1.0, 0.0], [-sin, 0.0, cos]]),
            YX => Matrix3x3::new([[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]]),
            YZ => Matrix3x3::new([[1.0, 0.0, 0.0], [0.0, cos, sin], [0.0, -sin, cos]]),
            ZX => Matrix3x3::new([[cos, 0.0, -sin], [0.0, 1.0, 0.0], [sin, 0.0, cos]]),
            ZY => Matrix3x3::new([[1.0, 0.0, 0.0], [0.0, cos, -sin], [0.0, sin, cos]]),
            _ => Matrix3x3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]),
        }
    }

    pub fn _get_rot_mat_2d(plane: RotationPlane, angle: f64) -> Matrix2x2 {
        let cos: f64 = angle.cos();
        let sin: f64 = angle.sin();

        use RotationPlane::*;

        match plane {
            XY => Matrix2x2::new([[cos, sin], [-sin, cos]]),
            YX => Matrix2x2::new([[cos, -sin], [sin, cos]]),
            _ => Matrix2x2::new([[1.0, 0.0], [0.0, 1.0]]),
        }
    }
}

pub trait Len {
    fn len(&self) -> f64;
    fn is_empty(&self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub struct Pos1D {
    pub x: f64,
}

impl ops::Add for Pos1D {
    type Output = Pos1D;

    fn add(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x + rhs.x;

        Self::Output { x }
    }
}

impl ops::Mul<f64> for Pos1D {
    type Output = Pos1D;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x * rhs;

        Self::Output { x }
    }
}

impl Len for Pos1D {
    fn len(&self) -> f64 {
        self.x
    }

    fn is_empty(&self) -> bool {
        self.x == 0.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos2D {
    pub x: f64,
    pub y: f64,
}

impl ops::Add for Pos2D {
    type Output = Pos2D;

    fn add(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x + rhs.x;
        let y: f64 = self.y + rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Sub for Pos2D {
    type Output = Pos2D;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x - rhs.x;
        let y: f64 = self.y - rhs.y;

        Self::Output { x, y }
    }
}

impl ops::Mul<f64> for Pos2D {
    type Output = Pos2D;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x * rhs;
        let y: f64 = self.y * rhs;

        Self::Output { x, y }
    }
}

impl Len for Pos2D {
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn is_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl Pos2D {
    /// Transform the rendered coordinates such that it is displayed in the center of the window
    pub fn to_screen_coords(self, scale: f64, size: PhysicalSize<u32>) -> Pos2D {
        self * scale
            + Pos2D {
                x: size.width as f64 / 2.0,
                y: size.height as f64 / 2.0,
            }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add for Pos3D {
    type Output = Pos3D;

    fn add(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x + rhs.x;
        let y: f64 = self.y + rhs.y;
        let z: f64 = self.z + rhs.z;

        Self::Output { x, y, z }
    }
}

impl ops::Mul<f64> for Pos3D {
    type Output = Pos3D;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x * rhs;
        let y: f64 = self.y * rhs;
        let z: f64 = self.z * rhs;

        Self::Output { x, y, z }
    }
}

// Use bitwise xor operator as cross product operator
impl ops::BitXor for Pos3D {
    type Output = Pos3D;

    fn bitxor(self, rhs: Pos3D) -> Self::Output {
        let x: f64 = self.y * rhs.z - self.z * rhs.y;
        let y: f64 = self.z * rhs.x - self.x * rhs.z;
        let z: f64 = self.x * rhs.y - self.y * rhs.x;

        Self::Output { x, y, z }
    }
}

// Use >> operator as a dot product operator
impl ops::Shr for Pos3D {
    type Output = f64;

    fn shr(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Len for Pos3D {
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn is_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
}

trait Transformation {
    fn swap(&self, first_index: usize, second_index: usize);
    fn mult_row(&self, index: usize, factor: f64);
    fn add_row(&self, first_index: usize, second_index: usize, factor: f64);
}

// trait Solve {
//     fn gauss(&self, argument: [f64; 3]) -> [f64; 3];
//     fn inverse(&self) -> Self;
//     fn transpose(&self) -> Self;
// }

// impl Transformation for [[f64; 3]; 3] {

//     fn swap(&self, first_index: usize, second_index: usize) {
//         let swap = self[first_index];
//         self[first_index] = self[second_index];
//         self[second_index] = swap;
//     }

//     fn mult_row(&self, index: usize, factor: f64) {
//         for (i, v) in self[index].into_iter().enumerate() {
//             self[index][i] = v * factor;
//         }
//     }

//     fn add_row(&self, first_index: usize, second_index: usize, factor: f64) {
//         for (i, _) in self[first_index].into_iter().enumerate() {
//             self[first_index][i] += self[second_index][i] * factor;
//         }
//     }
// }

// impl Transformation for [f64; 3] {
//     fn swap(&self, first_index: usize, second_index: usize) {
//         let swap = self[first_index];
//         self[first_index] = self[second_index];
//         self[second_index] = swap;
//     }

//     fn mult_row(&self, index: usize, factor: f64) {
//         self[index] *= factor;
//     }

//     fn add_row(&self, first_index: usize, second_index: usize, factor: f64) {
//         self[first_index] += self[second_index] * factor;
//     }
// }

// impl Solve for [[f64; 3]; 3] {
//     fn gauss(&self, args: [f64; 3]) -> [f64; 3] {
//         let rows = self.transpose();
//         let result =
//         {
//             // Row reduction
//             if rows[0][0] == 0.0 {
//                 if rows[1][0] != 0.0 {
//                     rows.swap(0, 1);
//                     args.swap(0, 1);
//                 } else if rows[2][0] != 0.0 {
//                     rows.swap(0, 2);
//                     args.swap(0, 2);
//                 } else {
//                     println!("Matrix not solvable, only zeroes in first column");
//                 }
//             }

//             // Reduce first entry of first row to 1
//             let factor = 1.0 / rows[0][0];
//             rows.mult_row(0, factor);
//             args.mult_row(0, factor);

//             // Reduce first entries of second and third row to 0
//             let factor = -rows[1][0];
//             rows.add_row(1, 0, factor);
//             args.add_row(1, 0, factor);

//             let factor = -rows[2][0];
//             rows.add_row(2, 0, factor);
//             args.add_row(2, 0, factor);

//             // Reduce second entry of second row to 1 if not zero
//             if rows[1][1] == 0.0 {
//                 if rows[2][1] != 0.0 {
//                     rows.swap(1, 2);
//                 } else {
//                     println!("Matrix not solvable, too many zeroes in second column");
//                 }
//             }

//             let factor = 1.0/rows[1][1];
//             rows.mult_row(1, factor);
//             args.mult_row(1, factor);

//             // Reduce other entries in second column to 0
//             let factor = -rows[0][1];
//             rows.add_row(0, 1, factor);
//             args.add_row(0, 1, factor);

//             let factor = -rows[2][1];
//             rows.add_row(2, 1, factor);
//             args.add_row(2, 1, factor);

//             // Reduce third entry in third row to 1 if not zero
//             if rows[2][2] == 0.0 {
//                 println!("Matrix not solvable, cannot solve last column");
//             }

//             let factor = 1.0 / rows[2][2];
//             rows.mult_row(2, factor);
//             args.mult_row(2, factor);

//             // Reduce other entries in third column to 0
//             let factor = -rows[0][2];
//             rows.add_row(0, 2, factor);
//             args.add_row(0, 2, factor);

//             let factor = -rows[1][2];
//             rows.add_row(1, 2, factor);
//             args.add_row(1, 2, factor);

//             args
//         };

//         result
//     }

//     fn inverse(&self) -> Self {
//         let args = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
//         let rows = *self;

//         let result = {
//             // Row reduction
//             if rows[0][0] == 0.0 {
//                 if rows[1][0] != 0.0 {
//                     rows.swap(0, 1);
//                     args.swap(0, 1);
//                 } else if rows[2][0] != 0.0 {
//                     rows.swap(0, 2);
//                     args.swap(0, 2);
//                 } else {
//                     println!("Matrix not solvable, only zeroes in first column");
//                 }
//             }

//             // Reduce first entry of first row to 1
//             let factor = 1.0 / rows[0][0];
//             rows.mult_row(0, factor);
//             args.mult_row(0, factor);

//             // Reduce first entries of second and third row to 0
//             let factor = -rows[1][0];
//             rows.add_row(1, 0, factor);
//             args.add_row(1, 0, factor);

//             let factor = -rows[2][0];
//             rows.add_row(2, 0, factor);
//             args.add_row(2, 0, factor);

//             // Reduce second entry of second row to 1 if not zero
//             if rows[1][1] == 0.0 {
//                 if rows[2][1] != 0.0 {
//                     rows.swap(1, 2);
//                 } else {
//                     println!("Matrix not solvable, too many zeroes in second column");
//                 }
//             }

//             let factor = 1.0/rows[1][1];
//             rows.mult_row(1, factor);
//             args.mult_row(1, factor);

//             // Reduce other entries in second column to 0
//             let factor = -rows[0][1];
//             rows.add_row(0, 1, factor);
//             args.add_row(0, 1, factor);

//             let factor = -rows[2][1];
//             rows.add_row(2, 1, factor);
//             args.add_row(2, 1, factor);

//             // Reduce third entry in third row to 1 if not zero
//             if rows[2][2] == 0.0 {
//                 println!("Matrix not solvable, cannot solve last column");
//             }

//             let factor = 1.0 / rows[2][2];
//             rows.mult_row(2, factor);
//             args.mult_row(2, factor);

//             // Reduce other entries in third column to 0
//             let factor = -rows[0][2];
//             rows.add_row(0, 2, factor);
//             args.add_row(0, 2, factor);

//             let factor = -rows[1][2];
//             rows.add_row(1, 2, factor);
//             args.add_row(1, 2, factor);

//             args
//         };

//         return result;
//     }

//     fn transpose(&self) -> Self {
//         let rows = [
//             [self[0][0], self[1][0], self[2][0]],
//             [self[0][1], self[1][1], self[2][1]],
//             [self[0][2], self[1][2], self[2][2]]
//         ];
//         return rows;
//     }
// }

#[derive(Clone, Copy, Debug)]
pub struct Pos4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl ops::Add for Pos4D {
    type Output = Pos4D;

    fn add(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x + rhs.x;
        let y: f64 = self.y + rhs.y;
        let z: f64 = self.z + rhs.z;
        let w: f64 = self.w + rhs.w;

        Self::Output { x, y, z, w }
    }
}

impl ops::Sub for Pos4D {
    type Output = Pos4D;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: f64 = rhs.x - self.x;
        let y: f64 = rhs.y - self.y;
        let z: f64 = rhs.z - self.z;
        let w: f64 = rhs.w - self.w;

        Self::Output { x, y, z, w }
    }
}

impl ops::Mul<f64> for Pos4D {
    type Output = Pos4D;

    fn mul(self, rhs: f64) -> Self::Output {
        let x: f64 = self.x * rhs;
        let y: f64 = self.y * rhs;
        let z: f64 = self.z * rhs;
        let w: f64 = self.w * rhs;

        Self::Output { x, y, z, w }
    }
}

impl ops::Div<f64> for Pos4D {
    type Output = Pos4D;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

// Use bitwise xor operator as cross product operator
impl ops::BitXor for Pos4D {
    type Output = Pos4D;

    fn bitxor(self, rhs: Pos4D) -> Self::Output {
        let x: f64 = self.y * rhs.z - self.z * rhs.y;
        let y: f64 = self.z * rhs.w - self.w * rhs.z;
        let z: f64 = self.w * rhs.x - self.x * rhs.w;
        let w: f64 = self.x * rhs.y - self.y * rhs.x;

        Pos4D { x, y, z, w }
    }
}

// Use >> operator as a dot product operator
impl ops::Shr for Pos4D {
    type Output = f64;

    fn shr(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

impl Len for Pos4D {
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    fn is_empty(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0
    }
}
