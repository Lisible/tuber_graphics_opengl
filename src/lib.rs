/*
* MIT License 
* Copyright (c) 2018 Clément SIBILLE 
*
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
use tuber::graphics::scene_renderer::SceneRenderer;

use tuber::scene::{SceneGraph, SceneNode, NodeValue};

type VertexIndex = gl::types::GLuint;

pub struct GLSceneRenderer {
    pending_meshes: Vec<Mesh>,
    pending_batches: Vec<RenderBatch>
}
impl GLSceneRenderer {
    /// Creates a new OpenGL scene renderer
    pub fn new() -> GLSceneRenderer {
        GLSceneRenderer {
            pending_meshes: vec!(),
            pending_batches: vec!()
        }
    }

    /// Renders a scene node
    fn render_scene_node(&mut self, scene_node: &SceneNode) {
        match scene_node.value() {
            NodeValue::RectangleNode(rectangle) => self.render_rectangle_node(rectangle),
            _ => println!("Node value of {} isn't renderable", scene_node.identifier())
        }
    }

    /// Render the pending meshes
    pub fn render(&mut self) {
        self.sort_meshes();
        self.batch_meshes();
        self.render_batches();
    }

    /// Sorts the meshes in order to batch them
    fn sort_meshes(&mut self) {
        // TODO
    }

    /// Batches the meshes together
    fn batch_meshes(&mut self) {
        for mesh in self.pending_meshes.iter() {
           // TODO
           // if mesh.attributes == pending_batches.last().attributes
           //   add to current batch
           // else
           //   create new batch
            
            if self.pending_batches.len() == 0 {
                let mut render_batch = RenderBatch::new();
                render_batch.add_mesh(mesh.clone());
                self.pending_batches.push(render_batch);
            } else {
                self.pending_batches.last_mut().unwrap().add_mesh(mesh.clone());
            }
        }

        self.pending_meshes.clear();
    }

    /// Renders the batches of meshes
    fn render_batches(&mut self) {
        for batch in self.pending_batches.iter_mut() {
            batch.render();
        }

        self.pending_batches.clear();
    }
    
    /// Renders a rectangle node
    fn render_rectangle_node(&mut self, rectangle: &tuber::graphics::Rectangle) {
        let mut mesh = Mesh::new();
        let indices = [0, 1, 2, 2, 0, 3];
        let vertices = [
            Vertex::with_values((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0)),
            Vertex::with_values((0.0, rectangle.height(), 0.0), (1.0, 1.0, 1.0), (0.0, 1.0)),
            Vertex::with_values((rectangle.width(), rectangle.height(), 0.0), (1.0, 1.0, 1.0), (1.0, 1.0)),
            Vertex::with_values((rectangle.width(), 0.0, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0))
        ];

        mesh.add_vertices(&vertices);
        mesh.add_indices(&indices);

        self.pending_meshes.push(mesh);
    }
}

impl SceneRenderer for GLSceneRenderer {
    fn render_scene(&mut self, scene: &SceneGraph) {
        use std::collections::HashSet;

        let mut stack = vec!(scene.root());
        let mut visited = HashSet::new();

        while stack.len() != 0 {
            if let Some(node) = stack.pop() {
                if !visited.contains(node.identifier()) {
                    self.render_scene_node(node);
                    visited.insert(node.identifier());
                    for child in node.children() {
                        stack.push(child);
                    }
                }
            }
        }

        self.render();
    }
}

/// Batch of meshes with the same attributes
struct RenderBatch {
    vao: opengl::VertexArrayObject,
    vbo: opengl::BufferObject,
    ebo: opengl::BufferObject,
    vertex_count: usize,
    index_count: usize,
    last_index: usize
}

impl RenderBatch {
    const MAX_BATCH_SIZE: usize = 1000;

    pub fn new() -> RenderBatch {
        let vao = opengl::VertexArrayObject::new();
        let vbo = opengl::BufferObject::with_size(gl::ARRAY_BUFFER,
                                                  RenderBatch::MAX_BATCH_SIZE);
        let ebo = opengl::BufferObject::with_size(gl::ELEMENT_ARRAY_BUFFER,
                                                  RenderBatch::MAX_BATCH_SIZE);

        vao.bind();
        vbo.bind();
        ebo.bind();
        vao.set_attribute(0, 3, gl::FLOAT, gl::FALSE,
                          std::mem::size_of::<Vertex>(),
                          std::ptr::null() as *const gl::types::GLvoid);
        vao.set_attribute(1, 3, gl::FLOAT, gl::FALSE,
                          std::mem::size_of::<Vertex>(),
                          (3 * std::mem::size_of::<f32>()) 
                          as *const gl::types::GLvoid);
        vao.set_attribute(2, 2, gl::FLOAT, gl::FALSE,
                          std::mem::size_of::<Vertex>(),
                          (6 * std::mem::size_of::<f32>()) 
                          as *const gl::types::GLvoid);
        vao.unbind();

        RenderBatch {
            vao,
            vbo,
            ebo,
            vertex_count: 0,
            index_count: 0,
            last_index: 0
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) {
        let mesh_vertex_count = mesh.vertices().len();
        let mesh_index_count = mesh.indices().len();

        self.vbo.bind();
        let mut vertex_buffer_pointer = self.vbo
            .map_buffer_range(self.vertex_count * std::mem::size_of::<Vertex>(), 
                              mesh_vertex_count * std::mem::size_of::<Vertex>(), 
                              gl::MAP_WRITE_BIT) as *mut Vertex;
        unsafe {
            for vertex in mesh.vertices().iter() {
                vertex_buffer_pointer.write(*vertex);
                vertex_buffer_pointer = vertex_buffer_pointer.offset(1);
            }
        }

        self.vbo.unmap();
        self.vbo.unbind();

        self.ebo.bind();
        let mut index_buffer_pointer = self.ebo
            .map_buffer_range(self.index_count * std::mem::size_of::<gl::types::GLuint>(),
                              mesh_index_count * std::mem::size_of::<gl::types::GLuint>(),
                              gl::MAP_WRITE_BIT) as *mut gl::types::GLuint;

        unsafe {
            let last_index = self.last_index;
            for index in mesh.indices().iter() {
                let index_offset = if last_index == 0 {
                    0
                } else {
                    last_index + 1
                };

                index_buffer_pointer.write(*index + index_offset as u32);
                dbg!(*index + index_offset as u32);

                if *index + index_offset as u32 > self.last_index as u32 {
                    self.last_index = (*index + index_offset as u32) as usize;
                }

                index_buffer_pointer = index_buffer_pointer.offset(1);
            }
        }

        self.ebo.unmap();
        self.ebo.unbind();

        self.vertex_count += mesh_vertex_count;
        self.index_count += mesh_index_count;
    }

    /// Renders the pending meshes
    pub fn render(&mut self) {
        self.vao.bind();
        opengl::draw_elements(gl::TRIANGLES,
                          self.index_count as gl::types::GLsizei,
                          gl::UNSIGNED_INT,
                          std::ptr::null() as *const gl::types::GLvoid);
    }
}

/// Represents a mesh
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>,
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
#[derive(Copy, Clone, Debug)]
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
