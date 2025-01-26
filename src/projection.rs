#![allow(unused_variables)]

use crate::matrix::*;
use crate::pos::*;

#[derive(Clone, Copy)]
pub struct Projection {
    pub r#type: ProjectionType,
    pub scale: f32,
}

impl Projection {
    pub fn new(r#type: ProjectionType, scale: f32) -> Self {
        Self {
            r#type, scale
        }
    }
}

#[derive(Clone, Copy)]
pub enum ProjectionType {
    Perspective,
    Stereographic,
    Collapse,
}

pub trait Project2D {
    type Output;

    fn project_2d(&self, projection: &Projection, screen_size: (usize, usize)) -> Self::Output;
}

pub trait Project3D {
    type Output;

    fn project_3d(&self, projection: &Projection, screen_size: (usize, usize)) -> Self::Output;
}

impl Projection {
    /// Get the position of the camera based on the type of projection
    pub fn get_camera_pos(&self) -> Pos3D {
        use self::ProjectionType::*;
        match self.r#type {
            Perspective => Pos3D {
                x: -1.0,
                y: 0.0,
                z: 0.0,
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

    pub fn project<T, U>(&self, pos: T, size: (usize, usize)) -> (U, f32) where T: Project2D<Output = (U, f32)> {
        pos.project_2d(&self, size)
    }
}

impl Project3D for Pos3D {
    type Output = Pos3D;

    fn project_3d(&self, projection: &Projection, screen_size: (usize, usize)) -> Self::Output {
        *self 
    }
}

impl Project3D for Pos4D {
    type Output = Pos3D;

    fn project_3d(&self, projection: &Projection, screen_size: (usize, usize)) -> Self::Output {
        use self::ProjectionType::*;
        match projection.r#type {
            Perspective => Pos3D {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            Stereographic => Pos3D {
                x: (self.x / (2.0 + self.w)),
                y: (self.y / (2.0 + self.w)),
                z: (self.z / (2.0 + self.w)),
            },
            Collapse => Pos3D {
                x: self.x,
                y: self.y,
                z: self.z,
            },
        }
    }
}

impl Project2D for Pos3D {
    type Output = (Pos2D, f32);

    fn project_2d(&self, projection: &Projection, screen_size: (usize, usize)) -> Self::Output {
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

        use self::ProjectionType::*;
        match projection.r#type {
            Perspective => {
                let bound = screen_size.0.min(screen_size.1) as f32 / 2.0;
                let zratio = 0.9 - (self.x / projection.scale) * 0.3;

                // Calculate the screen position of the pixel
                let screen_pos = Pos2D {
                    x: (screen_size.0 as f32 / 2.0 - zratio * bound * (self.z / projection.scale)).floor(),
                    y: (screen_size.1 as f32 / 2.0 + zratio * bound * (self.y / projection.scale)).floor(),
                };

                // Calculate the screen depth of the pixel
                let depth = {
                    10.0 / 2.0 - zratio * bound * (self.z / projection.scale)
                };

                (screen_pos, depth)
            }
            Stereographic => {
                let screen_pos = (SCREEN_MATRIX_3D * *self).to_screen_coords(projection.scale * 100.0, screen_size);
                let depth = 0.0;

                (screen_pos, depth)
            },
            Collapse => {
                let screen_pos = Pos2D { x: self.x, y: self.y }.to_screen_coords(projection.scale, screen_size);
                let depth = self.z / 10.0;

                (screen_pos, depth)
            },
        }
    }
}

impl Project2D for Pos4D {
    type Output = (Pos2D, f32);

    fn project_2d(&self, projection: &Projection, size: (usize, usize)) -> Self::Output {
        self.project_3d(projection, size).project_2d(projection, size)
    }
}
