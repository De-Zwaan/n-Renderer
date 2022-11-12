use std::f64::consts::PI;
use winit::dpi::PhysicalSize;

use crate::{pos::*, sterographic, matrix::*};

pub struct Object {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
pub struct Node {
    pub pos: Pos4D,
    pub r: f64,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub start_node: Pos4D,
    pub end_node: Pos4D,
    pub r: f64,
}

impl Object {
    pub fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: u64) {
        let angle = t as f64 * PI / 256.0;

        // Create a rotation_matrix using the rotation of the cube
        let cos = angle.cos();
        let sin = angle.sin();

        // let rotation_xz_matrix = Matrix4x4::new([[cos, 0.0, sin, 0.0], [0.0, 1.0, 0.0, 0.0], [-sin, 0.0, cos, 0.0], [0.0, 0.0, 0.0, 1.0]]);
        let rotation_zw_matrix = Matrix4x4::new([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, cos, sin], [0.0, 0.0, -sin, cos]]);
        
        let rotation_matrix = rotation_zw_matrix;

        // Loop over all edges
        for (_i, edge) in self.edges.iter().enumerate() {
            // Transform the start and end_node 
            let start_node = rotation_matrix * edge.start_node;
            let end_node = rotation_matrix * edge.end_node;

            Edge {start_node, end_node, r: edge.r}.draw(screen, size);
        }

        for (_i, node) in self.nodes.iter().enumerate() {
            let pos: Pos4D = rotation_matrix * node.pos;
            Node {pos: pos, r: node.r}.draw(screen, size);
        }
    }
}

trait Render {
    fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>);
}

impl Render for Node {
    fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>) {
        // Transform the Node to screen coordinates
        let pos: Pos2D = sterographic(self.pos, size);

        // Set the color of the points
        let rgba = [0xff, 0xaa, 0xff, 0xff];

        let r = 1.0 as i32; //(self.r as f64 * self.pos.w) as i32;

        // Draw small cubes around the point
        print_point(pos.x as i32, pos.y as i32, r as i32, screen, size, rgba);
    }
}

impl Render for Edge {
    fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>) {
        // Calculate the screen coordinates of the start and end points
        let start_point:    Pos2D = sterographic(self.start_node,   size);
        let end_point:      Pos2D = sterographic(self.end_node,     size);

        // Calculate vector for line connecting start and end point
        let edge = {
            [
                end_point.x - start_point.x,
                end_point.y - start_point.y,
            ]
        };

        // Set 1 / the amount of points that compose an edge
        let resolution: f64 = 0.01;

        let rgba = [0xff, 0x00, 0xbb, 0xff];

        for i in 0..=((1.0/resolution) as i32) {
            // let slope = (self.start_node.w.max(self.end_node.w) - self.start_node.w.min(self.end_node.w)) / (1.0 / resolution);
            // let r = ((self.r * self.start_node.w.min(self.end_node.w) + slope * i as f64)) as i32; 

            let x_p = (edge[0] * i as f64 * resolution) as i32 + start_point.x as i32;
            let y_p = (edge[1] * i as f64 * resolution) as i32 + start_point.y as i32;

            let r = 1.0 as i32;
            print_point(x_p, y_p, r, screen, size, rgba);
        }
    }
}

fn print_point(x: i32, y: i32, r: i32, screen: &mut [u8], size: PhysicalSize<u32>, color: [u8; 4]) {
    for x_off in -r..=r {
        for y_off in -r..=r {
            let x_p = x + x_off;
            let y_p = y + y_off;

            print_coord_in_pixelbuffer(x_p, y_p, screen, size, color)
        }
    }
}

fn print_coord_in_pixelbuffer(x : i32, y: i32, screen: &mut [u8], size: PhysicalSize<u32>, color: [u8; 4]) {
    // Calculate the index of the current coordinate
    if x <= size.width as i32 && x >= 0 && y <= size.height as i32 && y >= 0 {
        let i = (y * size.width as i32) as usize + x as usize;
    
        // Update for every color
        if i * 4 < screen.len() && i * 4 > 0 {
            for c in 0..3 {
                screen[i * 4 + c] = color[c];
            }
        }
    }
}

pub fn empty() -> Object {
    let nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();

    Object { nodes, edges }
}

pub fn create_3_cube(r: f64) -> Object {
    let points: [Pos4D; 8] = [
        Pos4D { x: r * -1.0, y: r * -1.0, z: r * -1.0, w: r * 0.0},
        Pos4D { x: r * -1.0, y: r * -1.0, z: r *  1.0, w: r * 0.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r * -1.0, w: r * 0.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r *  1.0, w: r * 0.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r * -1.0, w: r * 0.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r *  1.0, w: r * 0.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r * -1.0, w: r * 0.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r *  1.0, w: r * 0.0},
    ];

    Object {
        nodes: vec![
            Node { pos: points[00], r: 0.1 },
            Node { pos: points[01], r: 0.1 },
            Node { pos: points[02], r: 0.1 },
            Node { pos: points[03], r: 0.1 },
            Node { pos: points[04], r: 0.1 },
            Node { pos: points[05], r: 0.1 },
            Node { pos: points[06], r: 0.1 },
            Node { pos: points[07], r: 0.1 },
        ],
        edges: vec![
            Edge {start_node: points[00], end_node: points[01], r: 0.01},
            Edge {start_node: points[00], end_node: points[02], r: 0.01},
            Edge {start_node: points[00], end_node: points[04], r: 0.01},

            Edge {start_node: points[03], end_node: points[01], r: 0.01},
            Edge {start_node: points[03], end_node: points[02], r: 0.01},
            Edge {start_node: points[03], end_node: points[07], r: 0.01},

            Edge {start_node: points[05], end_node: points[01], r: 0.01},
            Edge {start_node: points[05], end_node: points[04], r: 0.01},
            Edge {start_node: points[05], end_node: points[07], r: 0.01},

            Edge {start_node: points[06], end_node: points[02], r: 0.01},
            Edge {start_node: points[06], end_node: points[04], r: 0.01},
            Edge {start_node: points[06], end_node: points[07], r: 0.01},
        ],
    }
}

pub fn create_4_cube(r: f64) -> Object {
    let points = [
        Pos4D { x: r * -1.0, y: r * -1.0, z: r * -1.0, w: r * -1.0},
        Pos4D { x: r * -1.0, y: r * -1.0, z: r * -1.0, w: r *  1.0},
        Pos4D { x: r * -1.0, y: r * -1.0, z: r *  1.0, w: r * -1.0},
        Pos4D { x: r * -1.0, y: r * -1.0, z: r *  1.0, w: r *  1.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r * -1.0, w: r * -1.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r * -1.0, w: r *  1.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r *  1.0, w: r * -1.0},
        Pos4D { x: r * -1.0, y: r *  1.0, z: r *  1.0, w: r *  1.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r * -1.0, w: r * -1.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r * -1.0, w: r *  1.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r *  1.0, w: r * -1.0},
        Pos4D { x: r *  1.0, y: r * -1.0, z: r *  1.0, w: r *  1.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r * -1.0, w: r * -1.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r * -1.0, w: r *  1.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r *  1.0, w: r * -1.0},
        Pos4D { x: r *  1.0, y: r *  1.0, z: r *  1.0, w: r *  1.0},
    ];

    Object {
        nodes: vec![
            Node { pos: points[00], r: 0.1 },
            Node { pos: points[01], r: 0.1 },
            Node { pos: points[02], r: 0.1 },
            Node { pos: points[03], r: 0.1 },
            Node { pos: points[04], r: 0.1 },
            Node { pos: points[05], r: 0.1 },
            Node { pos: points[06], r: 0.1 },
            Node { pos: points[07], r: 0.1 },
            Node { pos: points[08], r: 0.1 },
            Node { pos: points[09], r: 0.1 },
            Node { pos: points[10], r: 0.1 },
            Node { pos: points[11], r: 0.1 },
            Node { pos: points[12], r: 0.1 },
            Node { pos: points[13], r: 0.1 },
            Node { pos: points[14], r: 0.1 },
            Node { pos: points[15], r: 0.1 },
        ],
        edges: vec![
            Edge {start_node: points[00], end_node: points[01], r: 0.01},
            Edge {start_node: points[00], end_node: points[02], r: 0.01},
            Edge {start_node: points[00], end_node: points[04], r: 0.01},
            Edge {start_node: points[00], end_node: points[08], r: 0.01},

            Edge {start_node: points[03], end_node: points[01], r: 0.01},
            Edge {start_node: points[03], end_node: points[02], r: 0.01},
            Edge {start_node: points[03], end_node: points[07], r: 0.01},
            Edge {start_node: points[03], end_node: points[11], r: 0.01},

            Edge {start_node: points[05], end_node: points[01], r: 0.01},
            Edge {start_node: points[05], end_node: points[04], r: 0.01},
            Edge {start_node: points[05], end_node: points[07], r: 0.01},
            Edge {start_node: points[05], end_node: points[13], r: 0.01},

            Edge {start_node: points[06], end_node: points[02], r: 0.01},
            Edge {start_node: points[06], end_node: points[04], r: 0.01},
            Edge {start_node: points[06], end_node: points[07], r: 0.01},
            Edge {start_node: points[06], end_node: points[14], r: 0.01},

            Edge {start_node: points[09], end_node: points[01], r: 0.01},
            Edge {start_node: points[09], end_node: points[08], r: 0.01},
            Edge {start_node: points[09], end_node: points[11], r: 0.01},
            Edge {start_node: points[09], end_node: points[13], r: 0.01},

            Edge {start_node: points[10], end_node: points[02], r: 0.01},
            Edge {start_node: points[10], end_node: points[08], r: 0.01},
            Edge {start_node: points[10], end_node: points[11], r: 0.01},
            Edge {start_node: points[10], end_node: points[14], r: 0.01},

            Edge {start_node: points[12], end_node: points[04], r: 0.01},
            Edge {start_node: points[12], end_node: points[08], r: 0.01},
            Edge {start_node: points[12], end_node: points[13], r: 0.01},
            Edge {start_node: points[12], end_node: points[14], r: 0.01},

            Edge {start_node: points[15], end_node: points[07], r: 0.01},
            Edge {start_node: points[15], end_node: points[11], r: 0.01},
            Edge {start_node: points[15], end_node: points[13], r: 0.01},
            Edge {start_node: points[15], end_node: points[14], r: 0.01},
        ],
    }
}

pub fn create_3_sphere(res: i32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();

    let phi = PI * (3.0 - (5.0_f64).sqrt());

    for i in 0..res {
        let y = 1.0 - (i as f64 / (res - 1) as f64) * 2.0;
        let r = (1.0 - y * y).sqrt();

        let theta = phi * i as f64;

        let x = theta.cos() * r;
        let z = theta.sin() * r;

        nodes.push(Node { pos: Pos4D { x, y, z, w: 0.0 }, r: 1.0 })
    }

    // for (i, _) in nodes.iter().enumerate().step_by(2) {
    //     edges.push(Edge { start_node: nodes[i].pos, end_node: nodes[i + 1].pos, r: 1.0 });
    // }

    Object { nodes, edges }
}

pub fn create_4_sphere(res: i32, r: f64) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();

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

                nodes.push( Node { pos, r: 0.1 } );
            }
        } 
    }

    Object { nodes, edges }
}