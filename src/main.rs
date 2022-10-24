#![windows_subsystem = "windows"]
pub mod pos;
pub mod matrix;
pub mod shapes;

use std::f64::consts::PI;

use matrix::*;
use pos::*;

use pixels::{SurfaceTexture, PixelsBuilder, Error};
use shapes::{create_4_cube, create_3_sphere, create_3_cube, create_4_sphere};
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}, dpi::{LogicalSize, PhysicalSize}};
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

const SCALE: f64 = 80.0;

pub struct Object {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Clone, Copy)]
pub struct Node {
    pos: Pos4D,
    r: f64,
}

#[derive(Clone, Copy)]
pub struct Edge {
    start_node: Pos4D,
    end_node: Pos4D,
    r: f64,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Spinny Spinny")
        // .with_decorations(false)
        // .with_transparent(true)
        .with_always_on_top(true)
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .with_min_inner_size(LogicalSize::new(100, 100))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window);
    
    let mut pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
        .build()?;

    let mut t: u64 = 0;

    // let cube = create_3_cube();
    let hypercube: Object = create_4_cube();
    // let sphere = create_4_sphere(1000);

    event_loop.run(move | event, _, control_flow | {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // println!("Window closed");
                control_flow.set_exit();
            },
            Event::WindowEvent { 
                event: WindowEvent::Resized(new_size),
                .. 
            } => {
                // println!("Window resized");
                pixels.resize_buffer(new_size.width, new_size.height);
                pixels.resize_surface(new_size.width, new_size.height);
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                t += 1;

                let screen = pixels.get_frame();
                
                for (_i, p) in screen.chunks_exact_mut(4).enumerate() {
                    p.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
                }   

                // Draw objects                
                // sphere.draw(screen, window.inner_size(), t);
                hypercube.draw(screen, window.inner_size(), t);

                // Render result
                if pixels.render().map_err(|e| println!("pixels.render() failed: {}", e)).is_err() {
                    control_flow.set_exit();
                };
            },
            _ => ()
        }
    })
}

static SCREEN_MATRIX_3D: Matrix2x3 = Matrix2x3 {
    x: Pos3D { x: 0.866, y: 0.0, z: -0.866 },
    y: Pos3D { x: -0.5, y: -1.0, z: -0.5 },
};

// fn perspective(pos: Pos4D, size: PhysicalSize<u32>) -> Pos2D {
//     let scale = 2.0;
//     let bound = size.width.min(size.height) as f64 / 2.0;
//     let zratio = pos.z / scale;

//     Pos2D { 
//         x: (size.width as f64  / 2.0 + (0.9 + zratio * 0.3) * bound * (pos.x / scale)).floor(), 
//         y: (size.height as f64 / 2.0 - (0.9 + zratio * 0.3) * bound * (pos.y / scale)).floor(),
//     }
// }

fn sterographic(pos: Pos4D, size: PhysicalSize<u32>) -> Pos2D {
    let pos_4d = pos * (1.0 / pos.len());
    let pos_3d = Pos3D {
        x: pos_4d.x / (1.0 - pos_4d.w), 
        y: pos_4d.y / (1.0 - pos_4d.w), 
        z: pos_4d.z / (1.0 - pos_4d.w),
    };
    SCREEN_MATRIX_3D * pos_3d * SCALE + Pos2D { x: size.width as f64 / 2.0, y: size.height as f64 / 2.0 }
}   

impl Object {
    fn draw(&self, screen: &mut [u8], size: PhysicalSize<u32>, t: u64) {
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
        let resolution: f64 = 0.05;

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