use crate::{matrix::{Matrix3x3, Matrix4x4}, object::{Node, Object}, pos::{Pos3D, Pos4D}};

/// Trivial object transformations
pub trait Transform<M, T>
where
    Self: Sized,
{
    /// Rotate an object using a rotation matrix
    fn rotate(&self, rotation_matrix: M) -> Self;

    /// Move an object using a vector
    fn translate(&self, vector: T) -> Self;

    /// Scale an object using a 1D scalar
    fn scale(&self, scalar: f32) -> Self;
}

impl Transform<Matrix3x3, Pos3D> for Object<Pos3D> {
    fn rotate(&self, rotation_matrix: Matrix3x3) -> Self {
        let mut nodes = self.nodes.clone();

        nodes.iter_mut().for_each(|node| {
            *node = node.rotate(rotation_matrix);
        });

        Self { nodes, faces: self.faces.clone() }
    }

    fn translate(&self, vector: Pos3D) -> Self {
        let mut nodes = self.nodes.clone();

        nodes.iter_mut().for_each(|node| {
            *node = node.translate(vector);
        });

        Self { nodes, faces: self.faces.clone() }
    }

    fn scale(&self, scale: f32) -> Self {
       let mut nodes = self.nodes.clone();

       nodes.iter_mut().for_each(|node| {
            *node = node.scale(scale)
       });

       Self { nodes, faces: self.faces.clone() }
    }
}

impl Transform<Matrix3x3, Pos3D> for Node<Pos3D> {
    fn rotate(&self, rotation_matrix: Matrix3x3) -> Self{
        Self { pos: rotation_matrix * self.pos, color: self.color, r: self.r } 
    }

    fn translate(&self, vector: Pos3D) -> Self {
        Self { pos: self.pos + vector, color: self.color, r: self.r } 
    }

    fn scale(&self, scale: f32) -> Self {
        Self { pos: self.pos * scale, color: self.color, r: self.r }
    }
}

impl Transform<Matrix4x4, Pos4D> for Object<Pos4D> {
    fn rotate(&self, rotation_matrix: Matrix4x4) -> Self {
        let mut nodes = self.nodes.clone();

        nodes.iter_mut().for_each(|node| {
            *node = node.rotate(rotation_matrix);
        });

        Self { nodes, faces: self.faces.clone() }
    }

    fn translate(&self, vector: Pos4D) -> Self {
        let mut nodes = self.nodes.clone();

        nodes.iter_mut().for_each(|node| {
            *node = node.translate(vector);
        });

        Self { nodes, faces: self.faces.clone() }
    }

    fn scale(&self, scale: f32) -> Self {
       let mut nodes = self.nodes.clone();

       nodes.iter_mut().for_each(|node| {
            *node = node.scale(scale)
       });

       Self { nodes, faces: self.faces.clone() }
    }
}

impl Transform<Matrix4x4, Pos4D> for Node<Pos4D> {
    fn rotate(&self, rotation_matrix: Matrix4x4) -> Self{
        Self { pos: rotation_matrix * self.pos, color: self.color, r: self.r } 
    }

    fn translate(&self, vector: Pos4D) -> Self {
        Self { pos: self.pos + vector, color: self.color, r: self.r } 
    }

    fn scale(&self, scale: f32) -> Self {
        Self { pos: self.pos * scale, color: self.color, r: self.r }
    }
}
