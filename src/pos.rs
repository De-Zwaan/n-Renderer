use std::{ops, process::Output};

pub trait Len {
    fn len(&self) -> f64;
}

#[derive(Clone, Copy)]
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
}

#[derive(Clone, Copy)]
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

impl Len for Pos3D {
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Pos3D {
    pub fn transform_to_screen(&self, screen_matrix: [[f64; 2]; 3]) -> Pos2D {
        let x: f64 = self.x * screen_matrix[0][0] + self.y * screen_matrix[1][0] + self.z * screen_matrix[2][0];
        let y: f64 = self.y * screen_matrix[0][1] + self.y * screen_matrix[1][1] + self.z * screen_matrix[2][1];

        Pos2D { x, y }
    }

    pub fn transform_to_pos3d(&self, m: [[f64; 3]; 3]) -> Self {
        let x: f64 = self.x * m[0][0] + self.y * m[1][0] + self.z * m[2][0];
        let y: f64 = self.x * m[0][1] + self.y * m[1][1] + self.z * m[2][1];
        let z: f64 = self.x * m[0][2] + self.y * m[1][2] + self.z * m[2][2];
       
        Self { x, y, z } 
    }

    pub fn rotate_around_y(&self, _axis: [Pos3D; 2], angle: f64) -> Self {
        // Formulate rotation matrix
        let rotation_matrix: [[f64; 3]; 3] = [[angle.cos(), 0.0, angle.sin()], [0.0, 1.0, 0.0], [angle.sin(), 0.0, -angle.cos()]];
        // Rotate in local space
        self.transform_to_pos3d(rotation_matrix)
    }

    // fn _rotate() {
    //     // To test, set axis as y axis
    //     // Todo: take axis into account

    //     // Transform node to local space
    //     // Such that the axis is transformed to be the x-axis, and the other axis 

    //     // Calculate the new y axis
    //     let v_axis = {
    //         let x = _axis[1].x - _axis[0].x;
    //         let y = _axis[1].y - _axis[0].y;
    //         let z = _axis[1].z - _axis[0].z;

    //         let length = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt(); 
    //         [
    //             x / length,
    //             y / length,
    //             z / length,
    //         ]
    //     };

    //     // Formulate matrix transformation
    //     let a_matrix: [[f64; 3]; 3] = [[,,], [v_axis[0], v_axis[1], v_axis[2]], [,,]]; 
    //     // Apply transformation
    //     let local = self._transform_to_pos3d(a_matrix);

    //     // Formulate rotation matrix
    //     let rotation_matrix: [[f64; 3]; 3] = [[angle.cos(), 0.0, angle.sin()], [0.0, 1.0, 0.0], [angle.sin(), 0.0, -angle.cos()]];
    //     // Rotate in local space
    //     let rotated = local._transform_to_pos3d(rotation_matrix);

    //     // Transform node back to global space
    //     // use inverse matrix of a_matrix
    //     let a_inverse_matrix = a_matrix.inverse();
    //     rotated._transform_to_pos3d(a_inverse_matrix)
    // }
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

#[derive(Clone, Copy)] 
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

impl ops::Sub for Pos4D {
    type Output = Pos4D;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: f64 = self.x - rhs.x;
        let y: f64 = self.y - rhs.y;
        let z: f64 = self.z - rhs.z;
        let w: f64 = self.w - rhs.w;

        Self::Output { x, y, z, w }
    }
}

impl Len for Pos4D {
    fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }
}