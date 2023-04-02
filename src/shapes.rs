use std::f64::consts::PI;

use crate::{
    pos::Pos4D,
    render::{Color::*, Edge, Face, Node, Object},
};

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
    let mut nodes = Vec::new();

    for i in 0..=1 {
        let z = (i as f64 - 0.5) * 2.0 * r;
        for j in 0..=1 {
            let y = (j as f64 - 0.5) * 2.0 * r;
            for k in 0..=1 {
                let x = (k as f64 - 0.5) * 2.0 * r;
                nodes.push(Node {
                    pos: Pos4D { x, y, z, w: 0.0 },
                    r: 1.0,
                    color: RGB(i * 255, j * 255, k * 255),
                })
            }
        }
    }

    Object {
        nodes,
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
                node_a_index: 1,
                node_b_index: 3,
                node_c_index: 7,
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
    let mut nodes: Vec<Node> = Vec::new();

    // Generate the shape
    for i in 0..=1 {
        let w = (i as f64 - 0.5) * 2.0 * r;
        for j in 0..=1 {
            let z = (j as f64 - 0.5) * 2.0 * r;
            for k in 0..=1 {
                let y = (k as f64 - 0.5) * 2.0 * r;
                for l in 0..=1 {
                    let x = (l as f64 - 0.5) * 2.0 * r;
                    nodes.push(Node {
                        pos: Pos4D { x, y, z, w },
                        r: 1.0,
                        color: White,
                    })
                }
            }
        }
    }

    Object {
        nodes,
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
