use crate::{matrix::Matrix4x4, render::{Object, Node}, pos::Pos4D};

/// Trivial object transformations
pub trait Transform<T, V>
where
    Self: Sized,
{
    /// Rotate an object using a rotation matrix
    fn rotate(&mut self, rotation_matrix: T);

    /// Move an object using a vector
    fn r#move(&mut self, vector: V);

    /// Scale an object using a 1D scalar
    fn scale(&mut self, scalar: f32);
}

impl Transform<Matrix4x4, Pos4D> for Object {
    fn rotate(&mut self, rotation_matrix: Matrix4x4) {
        self.nodes
            .iter_mut()
            .for_each(|node| node.rotate(rotation_matrix));
    }

    fn r#move(&mut self, vector: Pos4D) {
        self.nodes.iter_mut().for_each(|node| node.r#move(vector));
    }

    fn scale(&mut self, scale: f32) {
        self.nodes.iter_mut().for_each(|node| node.scale(scale));
    }
}

impl Transform<Matrix4x4, Pos4D> for Node {
    fn rotate(&mut self, rotation_matrix: Matrix4x4) {
        self.pos = rotation_matrix * self.pos;
    }

    fn r#move(&mut self, vector: Pos4D) {
        self.pos = self.pos + vector;
    }

    fn scale(&mut self, scale: f32) {
        self.pos = self.pos * scale;
    }
}