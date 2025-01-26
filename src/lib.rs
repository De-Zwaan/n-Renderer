use std::{collections::{HashMap, HashSet}, isize};

use pos::Empty;
use object::{Node, Face, Object};
use render::{Color, Screen};

pub mod matrix;
pub mod pos;
pub mod projection;
pub mod render;
pub mod shapes;
pub mod transform;
pub mod object;

/// Print a point to the screen with a certain y(square) radius
pub fn print_point(
    x: usize,
    y: usize,
    r: usize,
    screen: &mut Screen,
    color: Color,
    depth: f32,
) {
    let rr = (r as f32 / 10.0) as isize;
    
    for x_off in -rr..=rr {
        let x_p = (x as isize + x_off) as usize;
        for y_off in -rr..=rr {
            let y_p = (y as isize + y_off) as usize;
            
            let _ = screen.write(x_p, y_p, color, depth);
        }
    }
}

pub fn remove_duplicates<T>(object: Object<T>) -> Object<T> where Node<T>: Eq + PartialEq + Clone, T: std::hash::Hash + Empty {
    let mut unique_nodes: HashMap<Node<T>, usize> = HashMap::new();

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
    let mut unique_nodes_ord: Vec<Node<T>> = vec![Node::empty(); unique_nodes.len()];

    // Use the new index to insert each node from the hashmap into the list
    unique_nodes.drain().for_each(|(node, index)| {
        unique_nodes_ord[index] = node;
    });

    Object { nodes: unique_nodes_ord, faces: remapped_faces.drain().collect() }
}
