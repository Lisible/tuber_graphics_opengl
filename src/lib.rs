/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE 
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

pub mod opengl;

type VertexIndex = gl::types::GLuint;

/// Represents a mesh
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>
}

impl Mesh {
    /// Creates a new, empty mesh
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec!(),
            indices: vec!()
        }
    }

    /// Adds vertices to the mesh
    ///
    /// # Examples
    ///
    /// ```
    /// use tuber_graphics_opengl::{Mesh, Vertex};
    ///
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.vertices().len(), 0);
    /// mesh.add_vertices(&[
    ///     Vertex::with_values((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0)),
    ///     Vertex::with_values((1.0, 0.0, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0)),
    ///     Vertex::with_values((0.0, 1.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0))
    /// ]);
    /// assert_eq!(mesh.vertices().len(), 3);
    /// ```
    pub fn add_vertices(&mut self, vertices: &[Vertex]) {
        self.vertices.extend_from_slice(vertices);
    }

    /// Adds indices to the mesh
    ///
    /// # Examples
    ///
    /// ```
    /// use tuber_graphics_opengl::Mesh;
    ///
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.indices().len(), 0);
    /// mesh.add_indices(&[0, 1, 2, 2, 0, 3]);
    /// assert_eq!(mesh.indices().len(), 6);
    /// ```
    pub fn add_indices(&mut self, indices: &[VertexIndex]) {
        self.indices.extend_from_slice(indices);
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<VertexIndex> {
        &self.indices
    }
}

/// Represents a vertex in 3D space
#[derive(Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
    texture_coordinates: (f32, f32)
}

impl Vertex {
    /// Creates a vertex with the given values
    pub fn with_values(position: (f32, f32, f32),
                       color: (f32, f32, f32),
                       texture_coordinates: (f32, f32)) -> Vertex {
        Vertex {
            position,
            color,
            texture_coordinates
        }
    }

    pub fn position(&self) -> (f32, f32, f32) {
        self.position
    }
    
    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn texture_coordinates(&self) -> (f32, f32) {
        self.texture_coordinates
    }
}
