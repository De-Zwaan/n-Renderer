use std::{f32::consts::PI, sync::{Arc, Mutex}};

// Crates for window managment
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder, error::EventLoopError,
};

// Actual rendering code
use n_renderer::{pos::{RotationAxis, RotationPlane}, projection::{Projection, ProjectionType::{self, *}}, render::Screen, shapes::*, transform::*};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const SCALE: f32 = 0.7;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();

    // Initialise the window
    let window = WindowBuilder::new()
        .with_title("Spinny Spinny")
        // .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(PhysicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Create a surface texture to render to
    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window,
    );

    // Create a pixelarray
    let mut pixels: pixels::Pixels = PixelsBuilder::new(WIDTH as u32, HEIGHT as u32, surface_texture).build().unwrap();
    
    // Create a pixelbuffer
    let screen = Arc::new(Mutex::new(Screen::new(WIDTH, HEIGHT)));

    let mut t: f32 = 0.0;

    // let shape = create_3_cube(0.5);
    // let shape = create_4_cube(1.0);
    // let mut shape = create_3_sphere(1000);
    let shape = create_4_sphere(1000, 1.8);
    // let mut shape = create_torus(100, 1.8);
    // let mut shape = empty();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event: Event<()>, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // println!("Window closed");
                control_flow.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(_new_size),
                ..
            } => {
                // println!("Window resized");
                // let _ = pixels.resize_buffer(new_size.width, new_size.height);
                // let _ = pixels.resize_surface(new_size.width, new_size.height);
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                {
                    let mut screen_lock = screen.lock().unwrap();
                    screen_lock.clear();
                }

                t += 0.1;

                // Transform the object
                let rotated_shape = shape.rotate(RotationPlane::get_rot_mat_4d(RotationPlane::WX, PI / 16.0 * t));

                // Draw the object
                rotated_shape.draw(Arc::clone(&screen), Projection::new(ProjectionType::Stereographic, 0.5 / SCALE));

                {
                    let screen_lock = screen.lock().unwrap();
                    let screen_slice = screen_lock.get_slice();
                    pixels.frame_mut().copy_from_slice(screen_slice);
                }
                                // Display the result on the screen
                if pixels
                    .render()
                    .map_err(|e| println!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    control_flow.exit();
                };
            },
            _ => (),
        }
    })
}
