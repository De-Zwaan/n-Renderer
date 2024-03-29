use std::f32::consts::PI;

// Crates for window managment
use pixels::{PixelsBuilder, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder, error::EventLoopError,
};

// Actual rendering code
use n_renderer::{pos::*, projection::Projection::*, render::*, shapes::*};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const SCALE: f32 = 200.0;

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();

    // Initialise the window
    let window = WindowBuilder::new()
        .with_title("Spinny Spinny")
        // .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(WIDTH as u32, HEIGHT as u32))
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

    // let mut t: u64 = 0;

    let mut shape = create_3_cube(1.0);
    // let mut shape = create_4_cube(1.0);
    // let mut shape = create_3_sphere(1000);
    // let mut shape = create_4_sphere(3200, 1.8);
    // let mut shape = create_torus(100, 1.8);
    // let mut shape = empty();

    // shape.rotate(RotationPlane::get_rot_mat_4d(RotationPlane::YZ, PI / 2.0));

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
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                // println!("Window resized");
                let _ = pixels.resize_buffer(new_size.width, new_size.height);
                let _ = pixels.resize_surface(new_size.width, new_size.height);
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let screen = pixels.frame_mut();
                let mut depth_buffer = [None; WIDTH * HEIGHT];

                // Create an empty pixelbuffer to render to
                screen.chunks_exact_mut(4).for_each(|p| {
                    p.copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
                });

                // Transform the object
                shape.rotate(RotationPlane::get_rot_mat_4d(RotationPlane::XZ, PI / 512.0));

                // Draw the object
                shape.draw(screen, &mut depth_buffer, window.inner_size(), SCALE, Perspective);

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
