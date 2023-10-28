use winit::dpi::PhysicalSize;

use crate::pos::*;
use crate::projection::Projection;
use crate::{matrix::*, print_coord_in_pixelbuffer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    White,
    Black,
    RGBA(u8, u8, u8, u8),
    RGB(u8, u8, u8),
    HSV(u16, u8, u8),
}

impl Color {
    pub fn get_rgba(&self) -> [u8; 4] {
        match self {
            Color::Red => [0xff, 0x00, 0x00, 0xff],
            Color::Orange => [0xff, 0xaa, 0x00, 0xff],
            Color::Yellow => [0xaa, 0xaa, 0x00, 0xff],
            Color::Green => [0x00, 0xff, 0x00, 0xff],
            Color::Blue => [0x00, 0x00, 0xff, 0xff],
            Color::Purple => [0xaa, 0x00, 0xaa, 0xff],
            Color::White => [0xff, 0xff, 0xff, 0xff],
            Color::Black => [0x00, 0x00, 0x00, 0xff],
            Color::RGBA(r, g, b, a) => [*r, *g, *b, *a],
            Color::RGB(r, g, b) => [*r, *g, *b, 0xff],
            Color::HSV(h, s, v) => {
                let region = 3.0 * (*h / 180) as f32;
                let c = *v as f32 * *s as f32;
                let x = c * (1.0 - (region % 2.0 - 1.0).abs());
        
                let (r, g, b) = {
                    if (0.0..1.0).contains(&region) {
                        (c, x, 0.0)
                    } else if (1.0..2.0).contains(&region) {
                        (x, c, 0.0)
                    } else if (2.0..3.0).contains(&region) {
                        (0.0, c, x)
                    } else if (3.0..4.0).contains(&region) {
                        (0.0, x, c)
                    } else if (4.0..5.0).contains(&region) {
                        (x, 0.0, c)
                    } else {
                        (c, 0.0, x)
                    }
                };
            
                [
                    ((r + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    ((g + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    ((b + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    0xff,
                ]
            }
        }
    }
}

pub struct Object {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Node {
    pub pos: Pos4D,
    pub color: Color,
    pub r: u32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Edge {
    pub start_node_index: usize,
    pub end_node_index: usize,
    pub r: u32,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Face {
    pub node_a_index: usize,
    pub node_b_index: usize,
    pub node_c_index: usize,
    pub r: u32,
}

impl Object {
    /// Draw all edges, vertices and faces of the object
    pub fn draw(
        &self,
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        projection_scale: f32,
        projection: Projection,
    ) {    
        self.edges.iter().for_each(|edge| {
            edge.draw(&self.nodes, screen, depth_buffer, size, projection, projection_scale);
        });
    
        self.nodes.iter().for_each(|node| {
            node.draw(&self.nodes, screen, depth_buffer, size, projection, projection_scale);
        });
    
        self.faces.iter().for_each(|face| {
            face.draw(&self.nodes, screen, depth_buffer, size, projection, projection_scale);
        });
    }
}

/// Trivial object transformations
pub trait Transform<T, V>
where
    Self: Sized,
{
    /// Rotate an object using a rotation matrix
    fn rotate(&mut self, rotation_matrix: T);

    /// Move an object using a vector
    fn r#move(&mut self, vector: V);

    /// Scale an object using a 1D scalar
    fn scale(&mut self, scalar: f32);
}

impl Transform<Matrix4x4, Pos4D> for Object {
    fn rotate(&mut self, rotation_matrix: Matrix4x4) {
        self.nodes
            .iter_mut()
            .for_each(|node| node.rotate(rotation_matrix));
    }

    fn r#move(&mut self, vector: Pos4D) {
        self.nodes.iter_mut().for_each(|node| node.r#move(vector));
    }

    fn scale(&mut self, scale: f32) {
        self.nodes.iter_mut().for_each(|node| node.scale(scale));
    }
}

impl Transform<Matrix4x4, Pos4D> for Node {
    fn rotate(&mut self, rotation_matrix: Matrix4x4) {
        self.pos = rotation_matrix * self.pos;
    }

    fn r#move(&mut self, vector: Pos4D) {
        self.pos = self.pos + vector;
    }

    fn scale(&mut self, scale: f32) {
        self.pos = self.pos * scale;
    }
}

trait Render {
    /// Determine the screen coordinates of objects using certain transformations and insert them into the pixelbuffer
    fn draw(
        &self,
        nodes: &[Node],
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f32,
    );

    /// Print a point to the screen with a certain y(square) radius
    fn print_point(
        x: i32,
        y: i32,
        z: f32,
        r: f32,
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        color: [u8; 4],
    ) {
        let rr = (r / 10.0) as i32;

        for x_off in -rr..=rr {
            for y_off in -rr..=rr {
                let x_p = x + x_off;
                let y_p = y + y_off;

                print_coord_in_pixelbuffer(x_p, y_p, z, screen, depth_buffer, size, color)
            }
        }
    }
}

fn scale(pos: Pos4D, to_camera: Pos3D) -> f32 {
    // Find the angle between the origin to the point and the origin to the camera
    let pos_3d: Pos3D = Pos3D {
        x: pos.x,
        y: pos.y,
        z: pos.z,
    };

    // The smaller the angle to the camera, the larger the nodes are when drawn to the screen
    // This is meant to simulate a kind of reflection, where the light is large and behind the camera
    (to_camera * (1.0 / to_camera.len())) >> ((pos_3d * (1.0 / pos_3d.len())) * 2.0)
}

impl Render for Node {
    #[allow(unused_variables)]
    fn draw(
        &self,
        nodes: &[Node],
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f32,
    ) {
        if self.r <= 0 {return;}
        // if self.pos.w != self.pos.w.clamp(0.9, 1.1) {return};

        // Transform the Node to screen coordinates
        let (pos, depth): (Pos2D, f32) = projection.project(self.pos, size, projection_scale);

        let r = self.r; //scale(self.pos, projection.get_camera_pos()) * self.r;

        // Set the color of the points
        let rgba = self.color.get_rgba();
        // rgba[2] = (50.0 * (self.pos.w + 2.5)) as u8;

        Self::print_point(pos.x as i32, pos.y as i32, depth as f32, r as f32, screen, depth_buffer, size, rgba);
    }
}

impl Render for Edge {
    fn draw(
        &self,
        nodes: &[Node],
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f32,
    ) {
        if self.r <= 0 {return;}

        let start_node: Node = nodes[self.start_node_index];
        let end_node: Node = nodes[self.end_node_index];

        // Calculate the screen coordinates of the start and end points
        let (screen_start_point, start_depth): (Pos2D, f32) = projection.project(start_node.pos, size, projection_scale);
        let (screen_end_point, end_depth): (Pos2D, f32) = projection.project(end_node.pos, size, projection_scale);

        // Calculate vector for line connecting start and end point
        let edge = {
            [
                screen_end_point.x - screen_start_point.x,
                screen_end_point.y - screen_start_point.y,
            ]
        };

        let to_camera = projection.get_camera_pos();

        // Calculate the radius of the start and end points of the edge
        let start_point_r = scale(start_node.pos, to_camera) * 0.1 * self.r as f32;
        let end_point_r = scale(end_node.pos, to_camera) * 0.1 * self.r as f32;

        // Set the amount of points that compose an edge based on the length of the edge on the screen
        let resolution: f32 = (screen_end_point - screen_start_point).len();

        // Interpolate between the colors of the two nodes
        let start_color = start_node.color.get_rgba();
        let end_color = end_node.color.get_rgba();

        for i in 0..=(resolution as i32) {
            let x_p = (edge[0] * i as f32 / resolution) as i32 + screen_start_point.x as i32;
            let y_p = (edge[1] * i as f32 / resolution) as i32 + screen_start_point.y as i32;

            let depth = ((end_depth - start_depth) * i as f32 / resolution) + start_depth;

            // Interpolate the radius of the points making up the edges
            let r =
                (((end_point_r - start_point_r) * i as f32 / resolution) + start_point_r) as f32;

            let mut rgba: [u8; 4] = [0; 4];
            for c in 0..4 {
                rgba[c] = (((end_color[c] as f32 - start_color[c] as f32) * i as f32 / resolution)
                    + start_color[c] as f32).max(0.0) as u8
            }

            // Change the blue channel of the edge based on the w coordiante
            // rgba[2] = (50.0
            //     * (((end_node.pos.w - start_node.pos.w) * i as f32 / resolution
            //         + start_node.pos.w)
            //         + 2.5)) as u8;

            Self::print_point(x_p, y_p, depth as f32, r, screen, depth_buffer, size, rgba);
        }
    }
}

impl Render for Face {
    fn draw(
        &self,
        nodes: &[Node],
        screen: &mut [u8],
        depth_buffer: &mut [Option<f32>],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f32,
    ) {
        let node_a = &nodes[self.node_a_index];
        let node_b = &nodes[self.node_b_index];
        let node_c = &nodes[self.node_c_index];

        let vector_a =
            projection.project_to_3d(node_b.pos) + projection.project_to_3d(node_a.pos) * -1.0;
        let vector_b =
            projection.project_to_3d(node_c.pos) + projection.project_to_3d(node_a.pos) * -1.0;

        // Get the normal vector of the surface by taking the cross product
        let normal: Pos3D = vector_a ^ vector_b;

        let to_camera = projection.get_camera_pos();

        // Let the brightness depend on the angle between the normal and the camera path
        // 1 if staight on, 0 if perpendicular and -1 if facing opposite
        let angle_to_camera: f32 = (normal >> to_camera) / (normal.len() * to_camera.len());

        if angle_to_camera < 0.0 {
            return;
        }

        // Get the locations of the three nodes of the triangle
        let (pos_a, depth_a): (Pos2D, f32) = projection.project(node_a.pos, size, projection_scale);
        let (pos_b, depth_b): (Pos2D, f32) = projection.project(node_b.pos, size, projection_scale);
        let (pos_c, depth_c): (Pos2D, f32) = projection.project(node_c.pos, size, projection_scale);

        // Calculate 2d vectors between the points on the screen
        let a_to_b: Pos2D = pos_b + (pos_a * -1.0);
        let a_to_c: Pos2D = pos_c + (pos_a * -1.0);

        // Change the alpha channel based on the angle between the camera and the surface
        let alpha = (255.0 * angle_to_camera.clamp(0.0, 1.0)) as u8;

        // Get the colors from the three nodes of the face
        let a_color = node_a.color.get_rgba();
        let b_color = node_b.color.get_rgba();
        let c_color = node_c.color.get_rgba();

        // Calculate the screen area of the face
        let area = 0.5
            * (Pos3D {
                x: a_to_b.x,
                y: a_to_b.y,
                z: 0.0,
            } ^ Pos3D {
                x: a_to_c.x,
                y: a_to_c.y,
                z: 0.0,
            })
            .len();

        let resolution: f32 = 0.5 * angle_to_camera.clamp(0.001, 1.0) * area.sqrt();

        // http://extremelearning.com.au/evenly-distributing-points-in-a-triangle/
        // let mut t: Vec<Pos2D> = Vec::new();

        // Define constants to generate points on a triangle
        // const G: f32 = 1.0 / 1.32471795572;
        // static ALPHA: Pos2D = Pos2D { x: G, y: G * G };

        // for n in 1..((1.0 / resolution) as i32) {
        //     t.push(ALPHA * n as f32)
        // }

        // for (_, p) in t.iter().enumerate() {
        //     let mut pos: Pos2D = Pos2D { x: 0.0, y: 0.0 };
        //     if p.x + p.y < 1.0 {
        //         pos = pos_a + (Pos2D { x: 1.0, y: 0.0 } * p.x * 10.0) + (Pos2D { x: 1.0, y: 1.0 } * p.y * 10.0);
        //     } else {
        //         pos = pos_a + (Pos2D { x: 1.0, y: 1.0 } * (1.0 - p.x * 10.0)) + (Pos2D { x: 1.0, y: 1.0 } * (1.0 - p.y * 10.0));
        //     }
        //     Self::print_point(pos.x as i32, pos.y as i32, self.r as i32, screen, size, rgba)
        // }

        // Amount of offset to add between the edges of the faces to avoid overlap
        let edge_offset = 0.35;

        // Iterate over points on the surface of the face and print them to the screen
        for k1 in 0..=(resolution as i32) {
            for k2 in 0..=(resolution as i32) {
                // Make sure it is a point on the triangle
                if (k1 as f32 + edge_offset) / resolution + (k2 as f32 + edge_offset) / resolution
                    > 1.0
                {
                    break;
                }

                let mut rgba: [u8; 4] = [0; 4];
                for c in 0..=3 {
                    rgba[c] = (a_color[c] as f32
                        + (b_color[c] as f32 - a_color[c] as f32)
                            * ((k1 as f32 + edge_offset) / resolution)
                        + (c_color[c] as f32 - a_color[c] as f32)
                            * ((k2 as f32 + edge_offset) / resolution))
                        as u8
                }

                rgba[3] = alpha;

                let p = pos_a
                    + a_to_b * ((k1 as f32 + edge_offset) / resolution)
                    + a_to_c * ((k2 as f32 + edge_offset) / resolution);

                let depth = depth_a + (depth_b - depth_a) * ((k1 as f32 + edge_offset) / resolution) + (depth_c - depth_a) * ((k2 as f32 + edge_offset) / resolution);

                Self::print_point(p.x as i32, p.y as i32, depth as f32, self.r as f32, screen, depth_buffer, size, rgba);
            }
        }
    }
}
