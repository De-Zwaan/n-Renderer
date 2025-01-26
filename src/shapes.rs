use std::f32::consts::PI;

use crate::{
    pos::{Pos3D, Pos4D},
    render::Color::*,
    object::{Face, Node, Object},
};

pub fn empty_3d() -> Object<Pos3D> {
    let nodes: Vec<Node<Pos3D>> = vec![
        Node {
            pos: Pos3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            r: 10,
            color: White,
        },
        Node {
            pos: Pos3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            r: 10,
            color: Red,
        },
        Node {
            pos: Pos3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            r: 10,
            color: Green,
        },
        Node {
            pos: Pos3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            r: 10,
            color: Blue,
        },
    ];

    let faces: Vec<Face> = Vec::new();

    Object {
        nodes,
        faces,
    }
}

pub fn empty_4d() -> Object<Pos4D> {
    let nodes: Vec<Node<Pos4D>> = vec![
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

    let faces: Vec<Face> = Vec::new();

    Object {
        nodes,
        faces,
    }
}

pub fn create_3_cube(r: f32) -> Object<Pos3D> {
    let mut nodes = Vec::new();

    for i in 0..=1 {
        let z = (i as f32 - 0.5) * 2.0 * r;

        for j in 0..=1 {
            let y = (j as f32 - 0.5) * 2.0 * r;

            for k in 0..=1 {
                let x = (k as f32 - 0.5) * 2.0 * r;

                nodes.push(Node {
                    pos: Pos3D { x, y, z },
                    r: 1,
                    color: RGB(i * 255, j * 255, k * 255),
                })
            }
        }
    }

    const FACE_INDECES: [(usize, usize, usize); 12] = [
        (0, 1, 4),
        (0, 4, 2),
        (0, 2, 1),
        (3, 1, 2),
        (3, 2, 7),
        (1, 3, 7),
        (5, 1, 7),
        (5, 4, 1),
        (5, 7, 4),
        (6, 2, 4),
        (6, 4, 7),
        (6, 7, 2),
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
        faces,
    }
}

pub fn create_4_cube(r: f32) -> Object<Pos4D> {
    let mut nodes: Vec<Node<Pos4D>> = Vec::new();

    // Generate the shape
    for i in 0..=1 {
        let x = (i as f32 - 0.5) * 2.0 * r;
        for j in 0..=1 {
            let y = (j as f32 - 0.5) * 2.0 * r;
            for k in 0..=1 {
                let z = (k as f32 - 0.5) * 2.0 * r;
                for l in 0..=1 {
                    let w = (l as f32 - 0.5) * 2.0 * r;
                    nodes.push(Node {
                        pos: Pos4D { x, y, z, w },
                        r: 10,
                        color: White,
                    })
                }
            }
        }
    }

    const FACE_INDECES: [(usize, usize, usize); 48] = [
        (0, 1, 4),
        (0, 4, 2),
        (0, 2, 1),
        (0, 8, 11),
        (0, 11, 13),
        (0, 13, 8),

        (3, 1, 2),
        (3, 2, 7),
        (3, 7, 1),
        (3, 8, 11),
        (3, 11, 14),
        (3, 14, 8),

        (5, 1, 7),
        (5, 4, 1),
        (5, 7, 4),
        (5, 8, 13),
        (5, 13, 14),
        (5, 14, 8),

        (6, 2, 4),
        (6, 4, 7),
        (6, 7, 2),
        (6, 11, 13),
        (6, 13, 14), 
        (6, 14, 11),

        (9, 8, 11),
        (9, 11, 2),
        (9, 2, 1),
        (9, 8, 11),
        (9, 11, 13),
        (9, 13, 8),

        (10, 1, 2),
        (10, 2, 7),
        (10, 7, 1),
        (10, 8, 11),
        (10, 11, 14),
        (10, 14, 8),

        (11, 1, 7),
        (11, 4, 1),
        (11, 7, 4),
        (11, 8, 13),
        (11, 13, 14),
        (11, 14, 8),

        (13, 2, 4),
        (13, 4, 7),
        (13, 7, 2),
        (13, 11, 13),
        (13, 13, 14),
        (13, 14, 11),
    ];
    let mut faces = Vec::new();
    for index in FACE_INDECES.iter() {
        faces.push(Face {
            node_a_index: index.0,
            node_b_index: index.1,
            node_c_index: index.2,
            r: 5,
        })
    }

    Object {
        nodes,
        faces,
    }
}

pub fn create_3_sphere(res: i32, scale: f32) -> Object<Pos3D> {
    let mut nodes: Vec<Node<Pos3D>> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    let phi = PI * (3.0 - (5.0_f32).sqrt());

    for i in 0..res {
        let y = 1.0 - (i as f32 / (res - 1) as f32) * 2.0;
        let r = (1.0 - y * y).sqrt();

        let theta = phi * i as f32;

        let x = theta.cos() * r;
        let z = theta.sin() * r;

        nodes.push(Node {
            pos: Pos3D { x, y, z } * scale,
            r: 10,
            color: Blue,
        })
    }

    Object {
        nodes,
        faces,
    }
}

pub fn create_4_sphere(res: i32, r: f32) -> Object<Pos4D> {
    let mut nodes: Vec<Node<Pos4D>> = Vec::new();
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
        faces,
    }
}

pub fn create_torus(res: i32, r: f32) -> Object<Pos3D> {
    let mut nodes: Vec<Node<Pos3D>> = Vec::new();
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

            let pos = Pos3D { x, y, z };

            nodes.push(Node {
                pos,
                r: 10,
                color: Purple,
            });
        }
    }

    Object {
        nodes,
        faces,
    }
}
