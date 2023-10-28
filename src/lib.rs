use winit::dpi::PhysicalSize;

pub mod matrix;
pub mod pos;
pub mod projection;
pub mod render;
pub mod shapes;

/// Change the pixel at coordinate (x, y) to the provided color. This will mutate the pixelbuffer.
pub fn print_coord_in_pixelbuffer(
    x: i32,
    y: i32,
    z: f32,
    screen: &mut [u8],
    depth_buffer: &mut [Option<f32>],
    size: PhysicalSize<u32>,
    color: [u8; 4],
) {
    if x < 0 || (x as u32) > size.width || y < 0 || (y as u32) > size.height {return;}

    // Calculate the index of the current coordinate
    let i = (y as usize * size.width as usize) as usize + x as usize;
    
    // If the index falls outside of the screen or depth buffer, return
    if i > (size.width * size.height) as usize {return;}

    if let &Some(depth) = &depth_buffer[i] {
        if depth > z {
            update_color(screen, i, color);

            // Update the depth buffer
            depth_buffer[i] = Some(z);
        }
    } else {
        update_color(screen, i, color);
        depth_buffer[i] = Some(z);
    }

    // Show depth buffer
    // let depth_value = (-z + 128.0).clamp(0.0, 256.0) as u8;
    // update_color(screen, i, [depth_value; 4]);
}

fn update_color(screen: &mut [u8], i: usize, color: [u8; 4]) {
    // Update for every color
    if i * 4 < screen.len() && i * 4 > 0 {
        for c in 0..=3 {
            screen[i * 4 + c] =
                (color[c] as u32).clamp(0, 255) as u8;
        }
    }
}
