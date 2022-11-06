#![windows_subsystem = "windows"]
pub mod pos;
pub mod matrix;
pub mod shapes;

use matrix::*;
use pos::*;

use pixels::{SurfaceTexture, PixelsBuilder, Error};
use shapes::{Object, create_4_cube, create_3_sphere, create_3_cube, create_4_sphere, Edge, Node};
use winit::{event_loop::EventLoop, window::WindowBuilder, event::{Event, WindowEvent}, dpi::{LogicalSize, PhysicalSize}};
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowBuilderExtWindows;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

const SCALE: f64 = 150.0;

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

    // let shape = create_3_cube();
    // let shape = create_4_cube();
    // let shape = create_3_sphere(1000);
    let shape = create_4_sphere(1000);

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
                shape.draw(screen, window.inner_size(), t);

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
        x: pos_4d.x / (1.0 + pos_4d.w), 
        y: pos_4d.y / (1.0 + pos_4d.w), 
        z: pos_4d.z / (1.0 + pos_4d.w),
    };
    SCREEN_MATRIX_3D * pos_3d * SCALE + Pos2D { x: size.width as f64 / 2.0, y: size.height as f64 / 2.0 }
}