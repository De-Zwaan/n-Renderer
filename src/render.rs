use std::usize;

use crate::object::{Node, Face};
use crate::pos::{Len, Pos2D, Pos3D};
use crate::projection::{Projection, Project2D, Project3D};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    White,
    Black,
    RGBA(u8, u8, u8, u8),
    RGB(u8, u8, u8),
    HSV(u16, u8, u8),
}

impl Color {
    pub fn get_rgba(&self) -> [u8; 4] {
        match self {
            Color::Red => [0xff, 0x00, 0x00, 0xff],
            Color::Orange => [0xff, 0xaa, 0x00, 0xff],
            Color::Yellow => [0xaa, 0xaa, 0x00, 0xff],
            Color::Green => [0x00, 0xff, 0x00, 0xff],
            Color::Blue => [0x00, 0x00, 0xff, 0xff],
            Color::Purple => [0xaa, 0x00, 0xaa, 0xff],
            Color::White => [0xff, 0xff, 0xff, 0xff],
            Color::Black => [0x00, 0x00, 0x00, 0xff],
            Color::RGBA(r, g, b, a) => [*r, *g, *b, *a],
            Color::RGB(r, g, b) => [*r, *g, *b, 0xff],
            Color::HSV(h, s, v) => {
                let region = 3.0 * (*h / 180) as f32;
                let c = *v as f32 * *s as f32;
                let x = c * (1.0 - (region % 2.0 - 1.0).abs());
        
                let (r, g, b) = {
                    if (0.0..1.0).contains(&region) {
                        (c, x, 0.0)
                    } else if (1.0..2.0).contains(&region) {
                        (x, c, 0.0)
                    } else if (2.0..3.0).contains(&region) {
                        (0.0, c, x)
                    } else if (3.0..4.0).contains(&region) {
                        (0.0, x, c)
                    } else if (4.0..5.0).contains(&region) {
                        (x, 0.0, c)
                    } else {
                        (c, 0.0, x)
                    }
                };
            
                [
                    ((r + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    ((g + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    ((b + (*v as f32 - c)) * 255.0).clamp(0.0, 255.0) as u8, 
                    0xff,
                ]
            }
        }
    }
}

pub struct Screen {
    color: Box<[[u8; 4]]>,
    depth: Box<[Option<f32>]>,
    width: usize,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            color: vec![[0x00; 4]; width * height].into_boxed_slice(),
            depth: vec![None; width * height].into_boxed_slice(),
            width,
        }
    }

    pub fn clear(&mut self) {
        self.color = vec![[0x00; 4]; self.color.len()].into_boxed_slice();
        self.depth = vec![None; self.depth.len()].into_boxed_slice();
    }

    #[allow(dead_code)]
    fn get_color(&self, x: usize, y: usize) -> Result<[u8; 4], &'static str> {
        let index = self.coord_to_index(x, y)?;
        match self.color.get(index) {
            Some(&p) => Ok(p),
            None => Err("Failed to get pixel color from screen"),
        }
    }

    fn get_depth(&self, x: usize, y: usize) -> Result<Option<f32>, &'static str> {
        let index = self.coord_to_index(x, y)?;
        match self.depth.get(index) {
            Some(&p) => Ok(p),
            None => Err("Failed to get pixel depth from screen"),
        }
    }

    fn put_color(&mut self, x: usize, y: usize, color: [u8; 4]) -> Result<(), &'static str> {
        let index = self.coord_to_index(x, y)?;

        if let Some(c) = self.color.get_mut(index) {
            *c= color;
            Ok(())
        } else {
            Err("Failed to put pixel onto screen")
        }
    }

    fn put_depth(&mut self, x: usize, y: usize, depth: f32) -> Result<(), &'static str> {
        let index = self.coord_to_index(x, y)?;
        
        if let Some(d) = self.depth.get_mut(index) {
            *d= Some(depth);
            Ok(())
        } else {
            Err("Failed to put pixel onto screen")
        }
    }

    pub fn write(&mut self, x: usize, y: usize, new_color: Color, new_depth: f32) -> Result<(), &'static str> {
        // Get the color and depth from the screen
        let old_depth = self.get_depth(x, y)?;

        // Test if the new pixel is in front of the old pixel
        if let Some(old_depth) = old_depth {
            if old_depth < new_depth {
                self.put_color(x, y, new_color.get_rgba())?;
                self.put_depth(x, y, new_depth)?;
            }
        } else {
            self.put_color(x, y, new_color.get_rgba())?;
            self.put_depth(x, y, new_depth)?;
        }

        Ok(())
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.color.len() / self.width)
    }

    fn coord_to_index(&self, x: usize, y: usize) -> Result<usize, &'static str> {
        if x > self.width || y > 1000000 {
            return Err("Trying to draw pixel outside screen buffer");
        }
        if self.color.len() < x * y || self.depth.len() < x * y {
            return Err("Trying to draw pixel outside screen buffer");
        }

        Ok(x + self.width * y)
    }

    pub fn get_slice(&self) -> &[u8] {
        // Flatten the 2D array of [u8; 4] to a 1D slice of u8
        let color_slice = &self.color;
        let flattened_slice = unsafe {
            std::slice::from_raw_parts(
                color_slice.as_ptr() as *const u8,
                color_slice.len() * 4,  // 4 u8 per pixel
            )
        };
        flattened_slice
    }
}

pub trait Render<P, PN, P2, P3> 
where 
    PN: Project2D<Output = (P2, f32)> + Project3D<Output = P3>,
{
    type Output;

    /// Determine the screen coordinates of objects using certain transformations and insert them into the pixelbuffer
    fn draw(
        &self,
        positions: &[P],
        screen_size: (usize, usize),
        projection: Projection,
    ) -> Self::Output;
}

impl<T> Render<Node<T>, T, Pos2D, Pos3D> for Node<T> 
where 
    T: Project2D<Output = (Pos2D, f32)> + Project3D<Output = Pos3D> + Copy, 
{
    type Output = Vec<(Pos2D, usize, Color, f32)>;
    
    fn draw(
        &self,
        _nodes: &[Node<T>],
        screen_size: (usize, usize),
        projection: Projection,
    ) -> Self::Output {
        let mut changes = Vec::new();

        if self.r == 0 { return changes; }
        // if self.pos.w != self.pos.w.clamp(0.9, 1.1) {return};

        // Transform the Node to screen coordinates
        let (pos, depth) = projection.project(self.pos, screen_size);

        let r = self.r; //scale(self.pos, projection.get_camera_pos()) * self.r;

        changes.push((pos, r, self.color, depth));

        changes
    }
}

impl<T> Render<Node<T>, T, Pos2D, Pos3D> for Face 
where 
    T: Project2D<Output = (Pos2D, f32)> + Project3D<Output = Pos3D> + Copy,
{
    type Output = Vec<(Pos2D, usize, Color, f32)>;
    
    fn draw(
        &self,
        nodes: &[Node<T>],
        screen_size: (usize, usize),
        projection: Projection,
    ) -> Self::Output {
        let mut changes = Vec::new();

        if self.r == 0 { return changes; }
        let node_a = &nodes[self.node_a_index];
        let node_b = &nodes[self.node_b_index];
        let node_c = &nodes[self.node_c_index];

        let vector_a =
            node_b.pos.project_3d(&projection, screen_size) + node_a.pos.project_3d(&projection, screen_size) * -1.0;
        let vector_b =
            node_c.pos.project_3d(&projection, screen_size) + node_a.pos.project_3d(&projection, screen_size) * -1.0;

        // Get the normal vector of the surface by taking the cross product
        let normal = vector_a ^ vector_b;

        let to_camera = projection.get_camera_pos();

        // Let the brightness depend on the angle between the normal and the camera path
        // 1 if staight on, 0 if perpendicular and -1 if facing opposite
        let angle_to_camera = (normal >> to_camera) / (normal.len() * to_camera.len());

        if angle_to_camera < 0.0 { return changes; }

        // Get the locations of the three nodes of the triangle
        let (pos_a, depth_a) = projection.project(node_a.pos, screen_size);
        let (pos_b, depth_b) = projection.project(node_b.pos, screen_size);
        let (pos_c, depth_c) = projection.project(node_c.pos, screen_size);

        // Calculate 2d vectors between the points on the screen
        let a_to_b = pos_b + (pos_a * -1.0);
        let a_to_c = pos_c + (pos_a * -1.0);

        // Change the alpha channel based on the angle between the camera and the surface
        let alpha = (255.0 * angle_to_camera.clamp(0.0, 1.0)) as u8;

        // Get the colors from the three nodes of the face
        let a_color = node_a.color.get_rgba();
        let b_color = node_b.color.get_rgba();
        let c_color = node_c.color.get_rgba();

        // Calculate triangle fill resolution based on angle to camera and length of edges
        let resolution: f32 = 0.2 * angle_to_camera.clamp(0.001, 1.0);
        let u_res: f32 = a_to_b.len() * resolution;
        let v_res: f32 = a_to_c.len() * resolution;

        // Amount of offset to add between the edges of the faces to avoid overlap
        let edge_offset = 0.35;

        // Iterate over points on the surface of the face and print them to the screen
        for k1 in 0..=(u_res as i32) {
            for k2 in 0..=(v_res as i32) {
                let u = (k1 as f32 + edge_offset) / u_res;
                let v = (k2 as f32 + edge_offset) / v_res;

                // Make sure it is a point on the triangle
                if u + v > 1.0 {break;}

                let mut rgba: [u8; 4] = [0; 4];
                for c in 0..4 {
                    rgba[c] = (a_color[c] as f32 + (b_color[c] as f32 - a_color[c] as f32) * u + (c_color[c] as f32 - a_color[c] as f32) * v) as u8
                }

                let color = Color::RGBA(rgba[0], rgba[1], rgba[2], alpha);

                let pos = pos_a + a_to_b * u + a_to_c * v;

                let depth = depth_a + (depth_b - depth_a) * u + (depth_c - depth_a) * v;

                changes.push((pos, self.r, color, depth));
            }
        }

        changes
    }
}
