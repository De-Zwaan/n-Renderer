use std::{sync::{Arc, Mutex}, thread, usize};

use crate::{pos::{Empty, Pos2D, Pos3D, Pos4D}, print_point, projection::{Project2D, Project3D, Projection}, render::{Color, Render, Screen}};

#[derive(Debug, Clone)]
pub struct Object<T> {
    pub nodes: Vec<Node<T>>,
    pub faces: Vec<Face>,
}

impl<T> Object<T> {
    pub fn new(nodes: Vec<Node<T>>, faces: Vec<Face>) -> Self {
        Self {
            nodes,
            faces,
        }
    }
}

impl From<Object<Pos3D>> for Object<Pos4D> {
    fn from(val: Object<Pos3D>) -> Self {
        let nodes = val.nodes.iter().map(|&node| node.into()).collect();

        Object { nodes, faces: val.faces }
    }
}

impl From<Object<Pos4D>> for Object<Pos3D> {
    fn from(val: Object<Pos4D>) -> Self {
        let nodes = val.nodes.iter().map(|&node| node.into()).collect();

        Object { nodes, faces: val.faces }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Node<T> {
    pub pos: T,
    pub color: Color,
    pub r: usize,
}

impl<T> Node<T> where T: std::hash::Hash + Empty {
    pub fn empty() -> Node<T> {
        Node {
            pos: T::empty(),
            color: Color::RGBA(0, 0, 0, 0),
            r: 0
        }
    }
}

impl From<Node<Pos4D>> for Node<Pos3D> {
    fn from(val: Node<Pos4D>) -> Node<Pos3D> {
        Node { pos: val.pos.into(), color: val.color, r: val.r }
    }
}

impl From<Node<Pos3D>> for Node<Pos4D> {
    fn from(val: Node<Pos3D>) -> Node<Pos4D> {
        Node { pos: val.pos.into(), color: val.color, r: val.r }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Face {
    pub node_a_index: usize,
    pub node_b_index: usize,
    pub node_c_index: usize,
    pub r: usize,
}

impl<T> Object<T> where T: Project2D<Output = (Pos2D, f32)> + Project3D<Output = Pos3D> + Into<Pos3D> + std::hash::Hash + Copy + std::marker::Sync + std::marker::Send + 'static {
    /// Draw all edges, vertices and faces of the object
    pub fn draw(
        &self,
        screen: Arc<Mutex<Screen>>,
        projection: Projection,
    ) {        
        let screen_size = screen.lock().unwrap().size();
        let num_threads = 4;
        let chunk_size = 200;

        let mut handles = Vec::with_capacity(num_threads);

        for chunk in self.nodes.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let cloned_screen = Arc::clone(&screen);
            let cloned_nodes = self.nodes.clone();

            let handle = thread::spawn(move || {
                let mut local_changes: Vec<(Pos2D, usize, Color, f32)> = Vec::new();

                for node in chunk {
                    local_changes.append(&mut node.draw(&cloned_nodes, screen_size, projection));
                }

                {
                    let mut screen = cloned_screen.lock().expect("Failed to lock the screen mutex");
                    for (pos, r, color, depth) in local_changes {
                        print_point(pos.x as usize, pos.y as usize, r, &mut *screen, color, depth);
                    }
                }
            });

            handles.push(handle);
        }

        for chunk in self.faces.chunks(chunk_size) {
            let chunk = chunk.to_vec();
            let cloned_screen = Arc::clone(&screen);
            let cloned_nodes = self.nodes.clone();

            let handle = thread::spawn(move || {
                let mut local_changes: Vec<(Pos2D, usize, Color, f32)> = Vec::new();

                for face in chunk {
                    local_changes.append(&mut face.draw(&cloned_nodes, screen_size, projection))
                }

                {
                    let mut screen = cloned_screen.lock().expect("Failed to lock the screen mutex");
                    for (pos, r, color, depth) in local_changes {
                        print_point(pos.x as usize, pos.y as usize, r, &mut *screen, color, depth);
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
