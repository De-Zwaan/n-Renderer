use std::f32::consts::PI;

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
            r: 10,
            color: White,
        },
        Node {
            pos: Pos4D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            r: 10,
            color: Red,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            },
            r: 10,
            color: Green,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            r: 10,
            color: Blue,
        },
        Node {
            pos: Pos4D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            r: 10,
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

pub fn create_3_cube(r: f32) -> Object {
    let mut nodes = Vec::new();

    for i in 0..=1 {
        let z = (i as f32 - 0.5) * 2.0 * r;

        for j in 0..=1 {
            let y = (j as f32 - 0.5) * 2.0 * r;

            for k in 0..=1 {
                let x = (k as f32 - 0.5) * 2.0 * r;

                nodes.push(Node {
                    pos: Pos4D { x, y, z, w: 0.0 },
                    r: 0,
                    color: RGB(i * 255, j * 255, k * 255),
                })
            }
        }
    }

    const EDGE_INDECES: [(usize, usize); 12] = [
        (0, 1), (0, 2), (0, 4), 
        (3, 1), (3, 2),         (3, 7), 
        (5, 1),         (5, 4), (5, 7),
                (6, 2), (6, 4), (6, 7),
    ];

    let mut edges = Vec::new();
    for index in EDGE_INDECES.iter() {
        edges.push(Edge {
            start_node_index: index.0,
            end_node_index: index.1,
            r: 0,
        })
    }

    const FACE_INDECES: [(usize, usize, usize); 12] = [
        (0, 1, 4), (0, 4, 2), (0, 2, 1), 
        (3, 1, 2), (3, 2, 7), (1, 3, 7),
        (5, 1, 7), (5, 4, 1), (5, 7, 4),
        (6, 2, 4), (6, 4, 7), (6, 7, 2),
    ];

    let mut faces = Vec::new();
    for index in FACE_INDECES.iter() {
        faces.push(Face {
            node_a_index: index.0,
            node_b_index: index.1,
            node_c_index: index.2,
            r: 20,
        })
    }

    Object {
        nodes,
        edges,
        faces,
    }
}

pub fn create_4_cube(r: f32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();

    // Generate the shape
    for i in 0..=1 {
        let w = (i as f32 - 0.5) * 2.0 * r;
        for j in 0..=1 {
            let z = (j as f32 - 0.5) * 2.0 * r;
            for k in 0..=1 {
                let y = (k as f32 - 0.5) * 2.0 * r;
                for l in 0..=1 {
                    let x = (l as f32 - 0.5) * 2.0 * r;
                    nodes.push(Node {
                        pos: Pos4D { x, y, z, w },
                        r: 10,
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
                r: 10,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 2,
                r: 10,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 4,
                r: 10,
            },
            Edge {
                start_node_index: 0,
                end_node_index: 8,
                r: 10,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 1,
                r: 10,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 2,
                r: 10,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 7,
                r: 10,
            },
            Edge {
                start_node_index: 3,
                end_node_index: 11,
                r: 10,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 1,
                r: 10,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 4,
                r: 10,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 7,
                r: 10,
            },
            Edge {
                start_node_index: 5,
                end_node_index: 13,
                r: 10,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 2,
                r: 10,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 4,
                r: 10,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 7,
                r: 10,
            },
            Edge {
                start_node_index: 6,
                end_node_index: 14,
                r: 10,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 1,
                r: 10,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 8,
                r: 10,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 11,
                r: 10,
            },
            Edge {
                start_node_index: 9,
                end_node_index: 13,
                r: 10,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 2,
                r: 10,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 8,
                r: 10,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 11,
                r: 10,
            },
            Edge {
                start_node_index: 10,
                end_node_index: 14,
                r: 10,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 4,
                r: 10,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 8,
                r: 10,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 13,
                r: 10,
            },
            Edge {
                start_node_index: 12,
                end_node_index: 14,
                r: 10,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 7,
                r: 10,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 11,
                r: 10,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 13,
                r: 10,
            },
            Edge {
                start_node_index: 15,
                end_node_index: 14,
                r: 10,
            },
        ],
        faces: Vec::new(),
    }
}

pub fn create_3_sphere(res: i32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let phi = PI * (3.0 - (5.0_f32).sqrt());

    for i in 0..res {
        let y = 1.0 - (i as f32 / (res - 1) as f32) * 2.0;
        let r = (1.0 - y * y).sqrt();

        let theta = phi * i as f32;

        let x = theta.cos() * r;
        let z = theta.sin() * r;

        nodes.push(Node {
            pos: Pos4D { x, y, z, w: 0.0 },
            r: 10,
            color: Purple,
        })
    }

    Object {
        nodes,
        edges,
        faces,
    }
}

pub fn create_4_sphere(res: i32, r: f32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let res_per_plane = (res as f32).sqrt() as i32;

    // XZ plane
    for i in 0..res_per_plane {
        let cos_t: f32 = ((2.0 * PI) / res_per_plane as f32 * i as f32).cos();
        let sin_t: f32 = ((2.0 * PI) / res_per_plane as f32 * i as f32).sin();

        // rotating Z plane
        for j in 0..res_per_plane {
            let cos_r: f32 = ((2.0 * PI) / res_per_plane as f32 * j as f32).cos();
            let sin_r: f32 = ((2.0 * PI) / res_per_plane as f32 * j as f32).sin();

            // rotating W plane
            for k in 0..res_per_plane {
                let cos_s: f32 = ((2.0 * PI) / res_per_plane as f32 * k as f32).cos();
                let sin_s: f32 = ((2.0 * PI) / res_per_plane as f32 * k as f32).sin();

                let x: f32 = r * sin_t * sin_r * cos_s;
                let z: f32 = r * sin_t * sin_r * sin_s;

                let y: f32 = r * sin_t * cos_r;
                let w: f32 = r * cos_t;

                let pos = Pos4D { x, y, z, w };

                nodes.push(Node {
                    pos,
                    r: 10,
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

pub fn create_torus(res: i32, r: f32) -> Object {
    let mut nodes: Vec<Node> = Vec::new();
    let edges: Vec<Edge> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let major_r: f32 = r;
    let minor_r: f32 = 0.5 * r;

    // XZ plane
    for t in 0..res {
        let cos_t: f32 = ((2.0 * PI) / res as f32 * t as f32).cos();
        let sin_t: f32 = ((2.0 * PI) / res as f32 * t as f32).sin();

        for p in 0..res {
            let cos_p: f32 = ((2.0 * PI) / res as f32 * p as f32).cos();
            let sin_p: f32 = ((2.0 * PI) / res as f32 * p as f32).sin();

            let x = (major_r + minor_r * cos_t) * sin_p;
            let y = (major_r + minor_r * cos_t) * cos_p;
            let z = minor_r * sin_t;
            let w = 0.0;

            let pos = Pos4D { x, y, z, w };

            nodes.push(Node {
                pos,
                r: 10,
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
