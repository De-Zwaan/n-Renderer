use std::f64::consts::PI;
use std::vec;
use winit::dpi::PhysicalSize;

use crate::pos::*;
use crate::projection::Projection;
use crate::shapes::Color::*;
use crate::{matrix::*, print_coord_in_pixelbuffer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
        }
    }
}

pub struct Object {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Pos4D,
    pub r: f64,
    pub color: Color,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.pos - other.pos).len() < 0.0001 && (self.r - other.r).abs() < 0.0001 && self.color == other.color
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub start_node_index: usize,
    pub end_node_index: usize,
    pub r: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub node_a_index: usize,
    pub node_b_index: usize,
    pub node_c_index: usize,
    pub r: f64,
}

impl Object {
    /// Draw all edges, vertices and faces of the object
    pub fn draw(
        &self,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        projection_scale: f64,
        projection: Projection,
    ) {
        // Iterate over all edges, vertices and faces of the object and draw them
        // self.edges.iter().for_each(|edge| {
        //     edge.draw(&self.nodes, screen, size, projection, projection_scale);
        // });

        // self.nodes.iter().for_each(|node| {
        //     node.draw(&self.nodes, screen, size, projection, projection_scale);
        // });

        self.faces.iter().for_each(|face| {
            face.draw(&self.nodes, screen, size, projection, projection_scale);
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
    fn scale(&mut self, scalar: f64);
}

impl Transform<Matrix4x4, Pos4D> for Object {
    fn rotate(&mut self, rotation_matrix: Matrix4x4) {
        self.nodes.iter_mut().for_each(|node| node.rotate(rotation_matrix));
    }

    fn r#move(&mut self, vector: Pos4D) {
        self.nodes.iter_mut().for_each(|node| node.r#move(vector));
    }

    fn scale(&mut self, scale: f64) {
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

    fn scale(&mut self, scale: f64) {
        self.pos = self.pos * scale;
    }
}

trait Render {
    /// Determine the screen coordinates of objects using certain transformations and insert them into the pixelbuffer
    fn draw(
        &self,
        nodes: &Vec<Node>,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f64,
    );

    /// Print a point to the screen with a certain y(square) radius
    fn print_point(
        x: i32,
        y: i32,
        r: i32,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        color: [u8; 4],
    ) {
        for x_off in -r..=r {
            for y_off in -r..=r {
                let x_p = x + x_off;
                let y_p = y + y_off;

                print_coord_in_pixelbuffer(x_p, y_p, screen, size, color)
            }
        }
    }
}

fn scale(pos: Pos4D, to_camera: Pos3D) -> f64 {
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
        nodes: &Vec<Node>,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f64,
    ) {
        if self.r as i32 <= 0 {
            return;
        };
        // if self.pos.w != self.pos.w.clamp(0.9, 1.1) {return};

        // Transform the Node to screen coordinates
        let pos: Pos2D = projection.project(self.pos, size, projection_scale);

        let r = self.r; //scale(self.pos, projection.get_camera_pos()) * self.r;

        // Set the color of the points
        let rgba = self.color.get_rgba();
        // rgba[2] = (50.0 * (self.pos.w + 2.5)) as u8;

        // Draw small cubes around the point
        if r < 0.4 {
            return;
        };

        Self::print_point(pos.x as i32, pos.y as i32, r as i32, screen, size, rgba);
    }
}

impl Render for Edge {
    fn draw(
        &self,
        nodes: &Vec<Node>,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f64,
    ) {
        if self.r as i32 <= 0 {
            return;
        };

        let start_node: Node = nodes[self.start_node_index];
        let end_node: Node = nodes[self.end_node_index];

        // Calculate the screen coordinates of the start and end points
        let screen_start_point: Pos2D = projection.project(start_node.pos, size, projection_scale);
        let screen_end_point: Pos2D = projection.project(end_node.pos, size, projection_scale);

        // Calculate vector for line connecting start and end point
        let edge = { [screen_end_point.x - screen_start_point.x, screen_end_point.y - screen_start_point.y] };

        let to_camera = projection.get_camera_pos();

        // Calculate the radius of the start and end points of the edge
        let start_point_r = scale(start_node.pos, to_camera) * self.r;
        let end_point_r = scale(end_node.pos, to_camera) * self.r;

        // Set the amount of points that compose an edge based on the length of the edge on the screen
        let resolution: f64 = (screen_end_point - screen_start_point).len();

        // Interpolate between the colors of the two nodes
        let start_color = start_node.color.get_rgba();
        let end_color = end_node.color.get_rgba();
        
        for i in 0..=(resolution as i32) {
            let x_p = (edge[0] * i as f64 / resolution) as i32 + screen_start_point.x as i32;
            let y_p = (edge[1] * i as f64 / resolution) as i32 + screen_start_point.y as i32;
            
            // Interpolate the radius of the points making up the edges
            let r =
            (((end_point_r - start_point_r) * i as f64 / resolution) + start_point_r) as i32;
            
            let mut rgba: [u8; 4] = [0; 4];
            for c in 0..=3 {
                rgba[c] = (((end_color[c] - start_color[c]) as f64 * i as f64 / resolution) + start_color[c] as f64) as u8
            };

            // Change the blue channel of the edge based on the w coordiante
            rgba[2] = (50.0
                * (((end_node.pos.w - start_node.pos.w) * i as f64 / resolution
                    + start_node.pos.w)
                    + 2.5)) as u8;

            Self::print_point(x_p, y_p, r, screen, size, rgba);
        }
    }
}

impl Render for Face {
    fn draw(
        &self,
        nodes: &Vec<Node>,
        screen: &mut [u8],
        size: PhysicalSize<u32>,
        projection: Projection,
        projection_scale: f64,
    ) {
        let node_a = nodes[self.node_a_index];
        let node_b = nodes[self.node_b_index];
        let node_c = nodes[self.node_c_index];

        let vector_a =
            projection.project_to_3d(node_b.pos) + projection.project_to_3d(node_a.pos) * -1.0;
        let vector_b =
            projection.project_to_3d(node_c.pos) + projection.project_to_3d(node_a.pos) * -1.0;

        // Get the normal vector of the surface by taking the cross product and normalise to a
        // length of 1
        let normal: Pos3D = vector_a ^ vector_b;
        let n_normal: Pos3D = normal * (1.0 / normal.len());

        let to_camera = projection.get_camera_pos();

        // Let the brightness depend on the angle between the normal and the camera path
        // 1 if staight on, 0 if perpendicular and -1 if facing opposite
        let angle_to_camera: f64 = n_normal >> (to_camera * (1.0 / to_camera.len()));

        if angle_to_camera < -0.01 {
            return;
        }

        // Get the locations of the three nodes of the triangle
        let pos_a: Pos2D = projection.project(node_a.pos, size, projection_scale);
        let pos_b: Pos2D = projection.project(node_b.pos, size, projection_scale);
        let pos_c: Pos2D = projection.project(node_c.pos, size, projection_scale);

        // Calculate 2d vectors between the points on the screen
        let a_to_b: Pos2D = pos_b + (pos_a * -1.0);
        let a_to_c: Pos2D = pos_c + (pos_a * -1.0);

        // Change the alpha channel based on the angle between the camera and the surface
        let alpha = 0xff; // (255.0 * angle_to_camera.clamp(0.0, 1.0)) as u8;

        // Get the colors from the three nodes of the face
        let a_color = node_a.color.get_rgba();
        let b_color = node_b.color.get_rgba();
        let c_color = node_c.color.get_rgba();

        // Calculate the screen area of the face 
        let area = 0.5 * (Pos3D { x: a_to_b.x, y: a_to_b.y, z: 0.0 } ^ Pos3D { x: a_to_c.x, y: a_to_c.y, z: 0.0 }).len();

        let resolution: f64 = angle_to_camera.clamp(0.001, 1.0) * area.sqrt();

        // http://extremelearning.com.au/evenly-distributing-points-in-a-triangle/
        // let mut t: Vec<Pos2D> = Vec::new();

        // Define constants to generate points on a triangle
        // const G: f64 = 1.0 / 1.32471795572;
        // static ALPHA: Pos2D = Pos2D { x: G, y: G * G };

        // for n in 1..((1.0 / resolution) as i32) {
        //     t.push(ALPHA * n as f64)
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
        let edge_offset = 0.5;

        // Iterate over points on the surface of the face and print them to the screen
        for k1 in 0..=((resolution) as i32) {
            for k2 in 0..=((resolution) as i32) {
                // Make sure it is a point on the triangle
                if (k1 as f64 + edge_offset) / resolution + (k2 as f64 + edge_offset) / resolution > 1.0 {
                    break;
                }

                let mut rgba: [u8; 4] = [0; 4];
                for c in 0..=3 {
                    rgba[c] = (a_color[c] as f64 + (b_color[c] as f64 - a_color[c] as f64) as f64 * ((k1 as f64 + edge_offset) / resolution) + (c_color[c] as f64 - a_color[c] as f64) as f64 * ((k2 as f64 + edge_offset) / resolution)) as u8
                };

                rgba[3] = alpha;

                let p =
                    pos_a + a_to_b * ((k1 as f64 + edge_offset) / resolution) + a_to_c * ((k2 as f64 + edge_offset) / resolution);

                Self::print_point(p.x as i32, p.y as i32, self.r as i32, screen, size, rgba);
            }
        }
    }
}

pub fn empty() -> Object {
    let nodes: Vec<Node> = vec![
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            r: 1.0,
            color: White,
        },
        Node {
            pos: Pos4D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            r: 1.0,
            color: Red,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            r: 1.0,
            color: Green,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            r: 1.0,
            color: Blue,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            r: 1.0,
            color: Purple,
        },
    ];

    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    Object {
        nodes,
        edges,
        faces,
    }
}

pub fn create_3_cube(r: f64) -> Object {
    let points: [Pos4D; 8] = [
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * 0.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * 0.0,
        },
    ];

    Object {
        nodes: vec![
            Node {
                pos: points[0],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[1],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[2],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[3],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[4],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[5],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[6],
                r: 0.1,
                color: Yellow,
            },
            Node {
                pos: points[7],
                r: 0.1,
                color: Yellow,
            },
        ],
        edges: vec![
            Edge {
                start_node_index: 0,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 7,
                r: 1.0,
            },
        ],
        faces: vec![
            Face {
                node_a_index: 0,
                node_b_index: 1,
                node_c_index: 4,
                r: 1.0,
            },
            Face {
                node_a_index: 0,
                node_b_index: 4,
                node_c_index: 2,
                r: 1.0,
            },
            Face {
                node_a_index: 0,
                node_b_index: 2,
                node_c_index: 1,
                r: 1.0,
            },
            Face {
                node_a_index: 3,
                node_b_index: 1,
                node_c_index: 2,
                r: 1.0,
            },
            Face {
                node_a_index: 3,
                node_b_index: 2,
                node_c_index: 7,
                r: 1.0,
            },
            Face {
                node_a_index: 3,
                node_b_index: 7,
                node_c_index: 1,
                r: 1.0,
            },
            Face {
                node_a_index: 5,
                node_b_index: 1,
                node_c_index: 7,
                r: 1.0,
            },
            Face {
                node_a_index: 5,
                node_b_index: 4,
                node_c_index: 1,
                r: 1.0,
            },
            Face {
                node_a_index: 5,
                node_b_index: 7,
                node_c_index: 4,
                r: 1.0,
            },
            Face {
                node_a_index: 6,
                node_b_index: 2,
                node_c_index: 4,
                r: 1.0,
            },
            Face {
                node_a_index: 6,
                node_b_index: 4,
                node_c_index: 7,
                r: 1.0,
            },
            Face {
                node_a_index: 6,
                node_b_index: 7,
                node_c_index: 2,
                r: 1.0,
            },
        ],
    }
}

pub fn create_4_cube(r: f64) -> Object {
    let points = [
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * -1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * -1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * -1.0,
            z: r * 1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * -1.0,
            w: r * 1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * -1.0,
        },
        Pos4D {
            x: r * 1.0,
            y: r * 1.0,
            z: r * 1.0,
            w: r * 1.0,
        },
    ];

    Object {
        nodes: vec![
            Node {
                pos: points[0],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[1],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[2],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[3],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[4],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[5],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[6],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[7],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[8],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[9],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[10],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[11],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[12],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[13],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[14],
                r: 1.0,
                color: White,
            },
            Node {
                pos: points[15],
                r: 1.0,
                color: White,
            },
        ],
        edges: vec![
            Edge {
                start_node_index: 0,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 8,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 11,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 13,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 14,
                r: 1.0,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 1,
                r: 1.0,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 8,
                r: 1.0,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 11,
                r: 1.0,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 13,
                r: 1.0,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 2,
                r: 1.0,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 8,
                r: 1.0,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 11,
                r: 1.0,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 14,
                r: 1.0,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 4,
                r: 1.0,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 8,
                r: 1.0,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 13,
                r: 1.0,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 14,
                r: 1.0,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 7,
                r: 1.0,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 11,
                r: 1.0,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 13,
                r: 1.0,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 14,
                r: 1.0,
            },
        ],
        faces: Vec::new(),
    }
}

pub fn create_3_sphere(res: i32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let phi = PI * (3.0 - (5.0_f64).sqrt());

    for i in 0..res {
        let y = 1.0 - (i as f64 / (res - 1) as f64) * 2.0;
        let r = (1.0 - y * y).sqrt();

        let theta = phi * i as f64;

        let x = theta.cos() * r;
        let z = theta.sin() * r;

        nodes.push(Node {
            pos: Pos4D { x, y, z, w: 0.0 },
            r: 1.0,
            color: Purple,
        })
    }

    Object {
        nodes,
        edges,
        faces,
    }
}

pub fn create_4_sphere(res: i32, r: f64) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let res_per_plane = (res as f64).sqrt() as i32;

    // XZ plane
    for i in 0..res_per_plane {
        let cos_t: f64 = ((2.0 * PI) / res_per_plane as f64 * i as f64).cos();
        let sin_t: f64 = ((2.0 * PI) / res_per_plane as f64 * i as f64).sin();

        // rotating Z plane
        for j in 0..res_per_plane {
            let cos_r: f64 = ((2.0 * PI) / res_per_plane as f64 * j as f64).cos();
            let sin_r: f64 = ((2.0 * PI) / res_per_plane as f64 * j as f64).sin();

            // rotating W plane
            for k in 0..res_per_plane {
                let cos_s: f64 = ((2.0 * PI) / res_per_plane as f64 * k as f64).cos();
                let sin_s: f64 = ((2.0 * PI) / res_per_plane as f64 * k as f64).sin();

                let x: f64 = r * sin_t * sin_r * cos_s;
                let z: f64 = r * sin_t * sin_r * sin_s;

                let y: f64 = r * sin_t * cos_r;
                let w: f64 = r * cos_t;

                let pos = Pos4D { x, y, z, w };

                nodes.push(Node {
                    pos,
                    r: 1.0,
                    color: Purple,
                });
            }
        }
    }

    Object {
        nodes,
        edges,
        faces,
    }
}

pub fn create_torus(res: i32, r: f64) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let major_r: f64 = r;
    let minor_r: f64 = 0.5 * r;

    // XZ plane
    for t in 0..res {
        let cos_t: f64 = ((2.0 * PI) / res as f64 * t as f64).cos();
        let sin_t: f64 = ((2.0 * PI) / res as f64 * t as f64).sin();

        for p in 0..res {
            let cos_p: f64 = ((2.0 * PI) / res as f64 * p as f64).cos();
            let sin_p: f64 = ((2.0 * PI) / res as f64 * p as f64).sin();

            let x = (major_r + minor_r * cos_t) * sin_p;
            let y = (major_r + minor_r * cos_t) * cos_p;
            let z = minor_r * sin_t;
            let w = 0.0;

            let pos = Pos4D { x, y, z, w };

            nodes.push(Node {
                pos,
                r: 1.0,
                color: Purple,
            });
        }
    }

    Object {
        nodes,
        edges,
        faces,
    }
}
