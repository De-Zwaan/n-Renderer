use winit::dpi::PhysicalSize;

pub mod matrix;
pub mod pos;
pub mod projection;
pub mod shapes;

/// Change the pixel at coordinate (x, y) to the provided color. This will mutate the pixelbuffer.
pub fn print_coord_in_pixelbuffer(
    x: i32,
    y: i32,
    screen: &mut [u8],
    size: PhysicalSize<u32>,
    color: [u8; 4],
) {
    // Calculate the index of the current coordinate
    if x <= size.width as i32 && x >= 0 && y <= size.height as i32 && y >= 0 {
        let i = (y * size.width as i32) as usize + x as usize;

        // Update for every color
        if i * 4 < screen.len() && i * 4 > 0 {
            for c in 0..=3 {
                screen[i * 4 + c] = (screen[i * 4 + c] as u32 + color[c] as u32).clamp(0, 255) as u8;
            }
        }
    }
}