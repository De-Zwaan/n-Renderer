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
        for x_off in -r..r {
            for y_off in -r..r {
                let x_p = pos.x as i32 + x_off;
                let y_p = pos.y as i32 + y_off;

                // Calculate the index of the current coordinate
                if x_p <= size.width as i32 && x_p >= 0 && y_p <= size.height as i32 && y_p >= 0 {
                    let i = (y_p * size.width as i32) as usize + x_p as usize;
                    
                    // Update for every color
                    if i * 4 < screen.len() && i * 4 > 0 {
                        for c in 0..3 {
                            screen[i * 4 + c] = rgba[c];
                        }
                    }
                }

            }
        }
    }
}

impl Render for Edge {
    fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>) {
        // Calculate the screen coordinates of the start and end points
        let start_point:    Pos2D = sterographic(self.start_node, size);
        let end_point:      Pos2D = sterographic(self.end_node, size);

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

            let r = 1.0 as i32;
            for x_off in -r..=r {
                for y_off in -r..=r {
                    let x_p = (edge[0] * i as f64 * resolution) as i32 + x_off + start_point.x as i32;
                    let y_p = (edge[1] * i as f64 * resolution) as i32 + y_off + start_point.y as i32;

                    // Calculate the index of the current coordinate
                    if x_p <= size.width as i32 && x_p >= 0 && y_p <= size.height as i32 && y_p >= 0 {
                        let i = (y_p * size.width as i32) as usize + x_p as usize;
                    
                        // Update for every color
                        if i * 4 < screen.len() && i * 4 > 0 {
                            for c in 0..3 {
                                screen[i * 4 + c] = rgba[c];
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_3_cube() -> Object {
    let points: [Pos4D; 8] = [
        Pos4D { x: -1.0, y: -1.0, z: -1.0, w:  0.0},
        Pos4D { x: -1.0, y: -1.0, z:  1.0, w:  0.0},
        Pos4D { x: -1.0, y:  1.0, z: -1.0, w:  0.0},
        Pos4D { x: -1.0, y:  1.0, z:  1.0, w:  0.0},
        Pos4D { x:  1.0, y: -1.0, z: -1.0, w:  0.0},
        Pos4D { x:  1.0, y: -1.0, z:  1.0, w:  0.0},
        Pos4D { x:  1.0, y:  1.0, z: -1.0, w:  0.0},
        Pos4D { x:  1.0, y:  1.0, z:  1.0, w:  0.0},
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

pub fn create_4_cube() -> Object {
    let points = [
        Pos4D { x: -1.0, y: -1.0, z: -1.0, w: -1.0},
        Pos4D { x: -1.0, y: -1.0, z: -1.0, w:  1.0},
        Pos4D { x: -1.0, y: -1.0, z:  1.0, w: -1.0},
        Pos4D { x: -1.0, y: -1.0, z:  1.0, w:  1.0},
        Pos4D { x: -1.0, y:  1.0, z: -1.0, w: -1.0},
        Pos4D { x: -1.0, y:  1.0, z: -1.0, w:  1.0},
        Pos4D { x: -1.0, y:  1.0, z:  1.0, w: -1.0},
        Pos4D { x: -1.0, y:  1.0, z:  1.0, w:  1.0},
        Pos4D { x:  1.0, y: -1.0, z: -1.0, w: -1.0},
        Pos4D { x:  1.0, y: -1.0, z: -1.0, w:  1.0},
        Pos4D { x:  1.0, y: -1.0, z:  1.0, w: -1.0},
        Pos4D { x:  1.0, y: -1.0, z:  1.0, w:  1.0},
        Pos4D { x:  1.0, y:  1.0, z: -1.0, w: -1.0},
        Pos4D { x:  1.0, y:  1.0, z: -1.0, w:  1.0},
        Pos4D { x:  1.0, y:  1.0, z:  1.0, w: -1.0},
        Pos4D { x:  1.0, y:  1.0, z:  1.0, w:  1.0},
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

pub fn create_4_sphere(res: i32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();

    let res_per_plane = res / 6;

    // XY plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: angle.cos(), y: angle.sin(), z: 0.0, w: 0.0 };
        nodes.push(Node { pos, r: 0.1 });
    }
    
    // XZ plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: angle.cos(), y: 0.0, z: angle.sin(), w: 0.0 };
        nodes.push(Node { pos, r: 0.1 });
    }

    // XW plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: angle.cos(), y: 0.0, z: 0.0, w: angle.sin() };
        nodes.push(Node { pos, r: 0.1 });
    }

    // YZ plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: 0.0, y: angle.cos(), z: angle.sin(), w: 0.0 };
        nodes.push(Node { pos, r: 0.1 });
    }

    // YW plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: 0.0, y: angle.cos(), z: 0.0, w: angle.sin() };
        nodes.push(Node { pos, r: 0.1 });
    }

    // ZW plane
    for i in 0..res_per_plane {
        let angle = (2.0 * PI) / res_per_plane as f64 * i as f64;
        let pos = Pos4D { x: 0.0, y: 0.0, z: angle.cos(), w: angle.sin() };
        nodes.push(Node { pos, r: 0.1 });
    }

    Object { nodes, edges }
}