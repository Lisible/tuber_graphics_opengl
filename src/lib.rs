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

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

use tuber::resources::ResourceLoader;

pub mod opengl;

pub type Position3D = (f32, f32, f32);
pub type Color = (f32, f32, f32);
pub type Position2D = (f32, f32);

#[derive(Clone)]
pub struct Vertex {
    position: Position3D,
    color: Color,
    texture_coordinates: Position2D
}

impl Vertex {
    /// Creates a new vertex
    pub fn new() -> Vertex {
        Vertex {
            position: (0.0, 0.0, 0.0),
            color: (0.0, 0.0, 0.0),
            texture_coordinates: (0.0, 0.0),
        }
    }

    pub fn with_values(position: Position3D, 
                       color: Color, 
                       texture_coordinates: Position2D) -> Vertex {
        Vertex {
            position,
            color,
            texture_coordinates
        }
    }

    pub fn set_position(&mut self, position: Position3D) {
        self.position = position;
    }
    pub fn position(&self) -> Position3D {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_texture_coordinates(&mut self, texture_coordinates: Position2D) {
        self.texture_coordinates = texture_coordinates;
    }
    pub fn texture_coordinates(&self) -> Position2D {
        self.texture_coordinates
    }
}

pub struct Mesh {
    vertices: Vec<Vertex>,
}

impl Mesh {
    /// Creates a new, empty mesh
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec!(),
        }
    }

    /// Adds vertices to the mesh from a slice
    ///
    /// # Examples
    /// ```
    /// use renderer::{Mesh, Vertex};
    ///
    /// let vertices = [
    ///  Vertex::with_values((1.0, 0.4, 2.0), (1.0, 0.0, 0.0), (1.0, 1.0)),
    ///  Vertex::with_values((3.1, 2.7, 8.3), (4.0, 1.0, 2.0), (4.0, 1.0)),
    /// ];
    ///
    /// let mut mesh = Mesh::new();
    /// assert_eq!(mesh.vertices().len(), 0);
    /// mesh.add_vertices(&vertices);
    /// assert_eq!(mesh.vertices().len(), 2);
    /// ```
    pub fn add_vertices(&mut self, vertices: &[Vertex]) {
        self.vertices.extend_from_slice(vertices);
    }
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}

pub struct Renderer {
    pending_batches: Vec<RenderBatch>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            pending_batches: vec!()
        }
    }

    /// Pushes a batch into the pending batch list
    pub fn push_batch(&mut self, batch: RenderBatch) {
        self.pending_batches.push(batch);
    }

    /// Renders the pending batches
    pub fn render(&mut self) {
        for batch in &self.pending_batches {
            println!("Render batch of size {}", batch.size());
            batch.vao().bind();
            unsafe {
                gl::DrawArrays(gl::TRIANGLES,
                               0,
                               batch.size() as i32);
            }
        }

        self.pending_batches.clear();
    }
}


pub struct RenderBatch {
    vao: opengl::VertexArrayObject,
    vbo: opengl::BufferObject,
    max_size: usize,
    size: usize
}

impl RenderBatch {
    pub fn new(max_size: usize) -> RenderBatch {
        let vao = opengl::VertexArrayObject::new();
        let vbo = opengl::BufferObject::with_size::<Vertex>(gl::ARRAY_BUFFER,
                                                            max_size,
                                                            gl::DYNAMIC_DRAW);

        vao.bind();
        vbo.bind();
        vao.enable_attribute(0);
        vao.enable_attribute(1);
        vao.enable_attribute(2);
        vao.configure_attribute(0, 
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                std::mem::size_of::<Vertex>(),
                                0);
        vao.configure_attribute(1, 
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                std::mem::size_of::<Vertex>(),
                                3 * std::mem::size_of::<f32>());
        vao.configure_attribute(2, 
                                2,
                                gl::FLOAT,
                                gl::FALSE,
                                std::mem::size_of::<Vertex>(),
                                6 * std::mem::size_of::<f32>());
        vao.unbind();

        RenderBatch {
            vao,
            vbo,
            max_size,
            size: 0
        }
    }

    pub fn add_mesh(&mut self, mesh: Mesh) -> Result<(), &'static str> {
        let vertices = mesh.vertices();
        let vertex_count = vertices.len();
        
        if self.size + vertex_count > self.max_size {
            return Err("The batch is full"); 
        }

        self.vbo.bind();
        self.vbo.update_data(self.size * std::mem::size_of::<Vertex>(),
                            vertex_count * std::mem::size_of::<Vertex>(),
                            vertices.as_ptr() as *const gl::types::GLvoid);
        self.vbo.unbind();

        self.size += vertex_count;

        Ok(())
    }

    pub fn vao(&self) -> &opengl::VertexArrayObject{
        &self.vao
    }

    pub fn vbo(&self) -> &opengl::BufferObject {
        &self.vbo
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct ShaderLoader {}
impl ResourceLoader<opengl::ShaderProgram> for ShaderLoader {
    fn load(identifier: &'static str) 
        -> Result<opengl::ShaderProgram, String> {
        let mut resource_file_path = String::from("data/");
        resource_file_path.push_str(identifier);
        resource_file_path.push_str(".jbb");

        let mut resource_file = File::open(resource_file_path)
            .expect("Couldn't open resource file");
        let mut resource_file_content = String::new();
        resource_file.read_to_string(&mut resource_file_content)
            .expect("Couldn't read resource file");
        
        let resource_metadata: serde_json::Value = 
            serde_json::from_str(&resource_file_content)
            .expect("Couldn't parse resource file");

        let vertex_shader_file = resource_metadata.get("vertex")
            .expect("Vertex shader not found").as_str().unwrap();
        let fragment_shader_file = resource_metadata.get("fragment")
            .expect("Fragment shader not found").as_str().unwrap();

        let vertex_file = "data/shaders/".to_owned() + vertex_shader_file;
        let vertex_path = Path::new(&vertex_file);
        let fragment_file = "data/shaders/".to_owned() + fragment_shader_file;
        let fragment_path = Path::new(&fragment_file);

        let vertex_shader = opengl::Shader::from_file(&vertex_path,
                                                      gl::VERTEX_SHADER)?;
        let fragment_shader = opengl::Shader::from_file(&fragment_path,
                                                      gl::FRAGMENT_SHADER)?;

        let shader = opengl::ShaderProgram::from_shaders(
            &[vertex_shader, fragment_shader]
        )?;

        Ok(shader)
    }
}
