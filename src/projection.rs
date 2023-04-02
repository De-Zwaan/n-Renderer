use winit::dpi::PhysicalSize;

use crate::matrix::*;
use crate::pos::*;
use crate::projection::Projection::*;

#[derive(Clone, Copy)]
pub enum Projection {
    Perspective,
    Stereographic,
    Collapse,
}

impl Projection {
    /// Get the position of the camera based on the type of projection
    pub fn get_camera_pos(&self) -> Pos3D {
        match self {
            Perspective => Pos3D {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            Stereographic => Pos3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Collapse => Pos3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }

    /// A function to project a 3D coordinate to a 2D coordinate using matrix or perspective projection
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_graphics::projection::Projection;
    ///
    /// let projection = Stereographic;
    /// let pos = Pos4D { x: 1.0, y: 1.0, z: 1.0 }
    /// let result = projection.project_to_3d(pos, size);
    /// assert_eq!(result, );
    /// ```
    pub fn project_to_3d(&self, pos: Pos4D) -> Pos3D {
        match self {
            Perspective => Pos3D {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
            Stereographic => Pos3D {
                x: (pos.x / (2.0 + pos.w)),
                y: (pos.y / (2.0 + pos.w)),
                z: (pos.z / (2.0 + pos.w)),
            },
            Collapse => Pos3D {
                x: pos.x,
                y: pos.y,
                z: pos.z,
            },
        }
    }

    /// A function to project a 3D coordinate to a 2D coordinate using matrix or perspective projection
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_graphics::projection::Projection;
    ///
    /// let projection = Stereographic;
    /// let pos = Pos3D { x: 1.0, y: 1.0, z: 1.0 }
    /// let result = projection.project_to_2d(pos, size, scale);
    /// assert_eq!(result, Pos2D { x: 0.0, y: -2.0 });
    /// ```
    pub fn project_to_2d(&self, pos: Pos3D, size: PhysicalSize<u32>, scale: f64) -> Pos2D {
        static SCREEN_MATRIX_3D: Matrix2x3 = Matrix2x3 {
            x: Pos3D {
                x: 0.866,
                y: 0.0,
                z: -0.866,
            },
            y: Pos3D {
                x: -0.5,
                y: -1.0,
                z: -0.5,
            },
        };

        match self {
            Perspective => {
                let scale = 2.0;
                let bound = size.width.min(size.height) as f64 / 2.0;
                let zratio = 0.9 + (pos.z / scale) * 0.3;

                Pos2D {
                    x: (size.width as f64 / 2.0 + zratio * bound * (pos.x / scale)).floor(),
                    y: (size.height as f64 / 2.0 - zratio * bound * (pos.y / scale)).floor(),
                }
            }
            Stereographic => (SCREEN_MATRIX_3D * pos).to_screen_coords(scale, size),
            Collapse => Pos2D { x: pos.x, y: pos.y }.to_screen_coords(scale, size),
        }
    }

    pub fn project(&self, pos: Pos4D, size: PhysicalSize<u32>, scale: f64) -> Pos2D {
        self.project_to_2d(self.project_to_3d(pos), size, scale)
    }
}
