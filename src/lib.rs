use std::collections::{HashMap, HashSet};

use pos::Pos4D;
use render::{Node, Edge, Face, Color, Object};
use winit::dpi::PhysicalSize;

pub mod matrix;
pub mod pos;
pub mod projection;
pub mod render;
pub mod shapes;
pub mod transform;

/// Change the pixel at coordinate (x, y) to the provided color. This will mutate the pixelbuffer.
pub fn print_coord_in_pixelbuffer(
    x: i32,
    y: i32,
    z: f32,
    screen: &mut Vec<u8>,
    depth_buffer: &mut Vec<Option<f32>>,
    size: PhysicalSize<u32>,
    color: [u8; 4],
) {
    if x < 0 || (x as u32) > size.width || y < 0 || (y as u32) > size.height {return;}

    // Calculate the index of the current coordinate
    let i = (y as usize * size.width as usize) as usize + x as usize;
    
    // If the index falls outside of the screen or depth buffer, return
    if i > (size.width * size.height) as usize {return;}

    // let depth_buffer = &mut depth_buffer.lock().unwrap();
    // let screen = &mut screen.lock().unwrap();

    if let Some(depth) = depth_buffer[i] {
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

fn update_color(screen: &mut Vec<u8>, i: usize, color: [u8; 4]) {
    // Update for every color
    if i * 4 < screen.len() && i * 4 > 0 {
        for c in 0..4 {
            screen[i * 4 + c] =
                (color[c] as u32).clamp(0, 255) as u8;
        }
    }
}

pub fn remove_duplicates(object: Object) -> Object {
    let mut unique_nodes: HashMap<Node, usize> = HashMap::new();

    let mut remapped_edges: HashSet<Edge> = HashSet::new();
    let mut remapped_faces: HashSet<Face> = HashSet::new();

    // Create a hashmap to store the remap between indices
    let mut nodes_remap: HashMap<usize, usize> = HashMap::new();
    
    object.nodes.into_iter().enumerate().for_each(|(orig_index, node)| {
        let new_index = if let Some(&index) = unique_nodes.get(&node) {
            index
        } else {
            unique_nodes.insert(node, unique_nodes.len());
            unique_nodes.len() - 1
        };

        nodes_remap.insert(orig_index, new_index);
    });

    // Adjust the node indices of the edges
    for mut edge in object.edges {
        if let Some(&new_index) = nodes_remap.get(&edge.start_node_index) {   
            edge.start_node_index = new_index;
        }

        if let Some(&new_index) = nodes_remap.get(&edge.end_node_index) {
            edge.end_node_index = new_index;
        }

        if edge.start_node_index >= unique_nodes.len() || edge.end_node_index >= unique_nodes.len() {
            println!("{:?}", edge)
        } else {
            remapped_edges.insert(edge);
        }
    }

    // Adjust the node indices of the faces
    for mut face in object.faces {
        if let Some(&new_index) = nodes_remap.get(&face.node_a_index) {
            face.node_a_index = new_index;
        }

        if let Some(&new_index) = nodes_remap.get(&face.node_b_index) {
            face.node_b_index = new_index;
        }

        if let Some(&new_index) = nodes_remap.get(&face.node_c_index) {
            face.node_c_index = new_index;
        }

        if face.node_a_index >= unique_nodes.len() || face.node_b_index >= unique_nodes.len() || face.node_c_index >= unique_nodes.len() {
            println!("{:?}", face)
        } else {
            remapped_faces.insert(face);
        }
    }

    // Create an empty vec to store the ordered list of unique nodes
    let mut unique_nodes_ord = vec![Node {pos: Pos4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }, color: Color::White, r: 0}; unique_nodes.len()];

    // Use the new index to insert each node from the hashmap into the list
    unique_nodes.drain().for_each(|(node, index)| {
        unique_nodes_ord[index] = node;
    });

    Object { nodes: unique_nodes_ord, edges: remapped_edges.drain().collect(), faces: remapped_faces.drain().collect() }
}