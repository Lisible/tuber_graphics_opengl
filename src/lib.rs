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
use std::rc::Rc;
use std::cell::RefCell;

use tuber::graphics::scene_renderer::SceneRenderer;
use tuber::resources::ResourceStore;

use tuber::scene::{SceneGraph, SceneNode, NodeValue};

pub mod opengl;
pub mod font;

type VertexIndex = gl::types::GLuint;

pub struct GLSceneRenderer {
    pending_meshes: Vec<Mesh>,
    pending_batches: Vec<RenderBatch>,
    texture_store: Rc<RefCell<ResourceStore<opengl::Texture>>>,
    font_store: Rc<RefCell<ResourceStore<font::Font>>>
}
impl GLSceneRenderer {
    /// Creates a new OpenGL scene renderer
    pub fn new(texture_store: Rc<RefCell<ResourceStore<opengl::Texture>>>,
               font_store: Rc<RefCell<ResourceStore<font::Font>>>) -> GLSceneRenderer {
        GLSceneRenderer {
            pending_meshes: vec!(),
            pending_batches: vec!(),
            texture_store,
            font_store
        }
    }

    /// Renders a scene node
    fn render_scene_node(&mut self, scene_node: &SceneNode) {
        match scene_node.value() {
            NodeValue::RectangleNode(rectangle) => self.render_rectangle_node(rectangle),
            NodeValue::LineNode(line) => self.render_line_node(line),
            NodeValue::SpriteNode(sprite) => self.render_sprite_node(sprite),
            NodeValue::TextNode(text) => self.render_text_node(text),
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
        self.pending_meshes.sort_by_key(|mesh| mesh.attributes());
    }

    /// Batches the meshes together
    fn batch_meshes(&mut self) {
        for mesh in self.pending_meshes.iter() {
            if (self.pending_batches.len() == 0) || 
                (self.pending_batches.last().unwrap().mesh_attributes() != mesh.attributes()) {
                
                let mut render_batch = RenderBatch::new(mesh.attributes().clone());
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
        println!("Batches to be rendered: {}", self.pending_batches.len());
        for batch in self.pending_batches.iter_mut() {
            let attributes = batch.mesh_attributes();

            if let Some(font_identifier) = attributes.font_identifier() {
                let font_store = self.font_store.borrow();
                let font = font_store.get(font_identifier).unwrap();
                opengl::enable_font_blending();
                font.bind_texture();
            }
            else if let Some(texture_identifier) = attributes.texture_identifier() {
                let texture_store = self.texture_store.borrow();
                let texture = texture_store.get(texture_identifier).unwrap();
                texture.bind();
            }

            batch.render();
        }

        self.pending_batches.clear();
    }

    fn render_rectangle_node(&mut self, rectangle: &tuber::graphics::Rectangle) {
        let mut mesh = Mesh::new(MeshAttributes::defaults());

        let c = rectangle.color();
        let indices = [0, 1, 2, 2, 0, 3];
        let vertices = [
            Vertex::with_values((0.0, 0.0, 0.0), (c.0, c.1, c.2), (0.0, 0.0)),
            Vertex::with_values((0.0, rectangle.height(), 0.0), (c.0, c.1, c.2), (0.0, 1.0)),
            Vertex::with_values((rectangle.width(), rectangle.height(), 0.0), (c.0, c.1, c.2), (1.0, 1.0)),
            Vertex::with_values((rectangle.width(), 0.0, 0.0), (c.0, c.1, c.2), (1.0, 0.0))
        ];

        mesh.add_vertices(&vertices);
        mesh.add_indices(&indices);

        self.pending_meshes.push(mesh);
    }

    fn render_sprite_node(&mut self, sprite: &tuber::graphics::Sprite) {
        let mesh_attributes = MeshAttributesBuilder::new()
            .texture(sprite.texture_identifier())
            .build();
        let mut mesh = Mesh::new(mesh_attributes);
       
        let indices = [0, 1, 2, 2, 0, 3];
        let vertices = [
            Vertex::with_values((0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0)),
            Vertex::with_values((0.0, sprite.height(), 0.0), (1.0, 1.0, 1.0), (0.0, 1.0)),
            Vertex::with_values((sprite.width(), sprite.height(), 0.0), (1.0, 1.0, 1.0), (1.0, 1.0)),
            Vertex::with_values((sprite.width(), 0.0, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0))
        ];

        mesh.add_vertices(&vertices);
        mesh.add_indices(&indices);

        self.pending_meshes.push(mesh);
    }

    fn render_text_node(&mut self, text: &tuber::graphics::Text) {
        let font_store = self.font_store.borrow();
        let font = font_store.get(text.font_identifier()).unwrap();

        let mut cursor_offset = 0.0;
        for c in text.text() {
            let character_metadata = font.metadata().character(c).unwrap();

            let mesh_attributes = MeshAttributesBuilder::new()
                .font(text.font_identifier())
                .build();


            let tw = 1024.0;
            let th = 1024.0;
            let x = character_metadata.x_coordinate() / tw;
            let y = -character_metadata.y_coordinate() / th;
            let y_off = -character_metadata.y_offset() / th;
            let w = character_metadata.width() / tw;
            let h = -character_metadata.height() / th;


            println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);

            let mut mesh = Mesh::new(mesh_attributes);
            let indices = [0, 1, 2, 2, 0, 3];
            let vertices = [
                Vertex::with_values((cursor_offset, y_off, 0.0), (1.0, 1.0, 1.0), (x, y)),
                Vertex::with_values((cursor_offset, y_off + h, 0.0), (1.0, 1.0, 1.0), (x, y + h)),
                Vertex::with_values((cursor_offset + w, y_off + h, 0.0), (1.0, 1.0, 1.0), (x + w, y + h)),
                Vertex::with_values((cursor_offset + w, y_off, 0.0), (1.0, 1.0, 1.0), (x + w, y))
            ];

            cursor_offset += w;

            mesh.add_vertices(&vertices);
            mesh.add_indices(&indices);
            self.pending_meshes.push(mesh);
        }
    }

    fn render_line_node(&mut self, line: &tuber::graphics::Line) {
        let mesh_attributes = MeshAttributesBuilder::new()
            .draw_mode(gl::LINES)
            .build();
        let mut mesh = Mesh::new(mesh_attributes);

        let indices = [0, 1];
        let vertices = [
            Vertex::with_values(line.first_point(), (1.0, 1.0, 1.0), (0.0, 0.0)),
            Vertex::with_values(line.second_point(), (1.0, 1.0, 1.0), (0.0, 0.0))
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

/// Builder for MeshAttributes
///
/// # Examples
///
/// ```
/// use tuber_graphics_opengl::MeshAttributesBuilder;
///
/// let mut builder = MeshAttributesBuilder::new()
///     .texture("textureA".into());
/// let configuration = builder.build();
/// match configuration.texture_identifier() {
///     Some(identifier) => assert_eq!(identifier, "textureA"),
///     None => assert!(false)
/// };
/// ```
pub struct MeshAttributesBuilder {
    texture_identifier: Option<String>,
    font_identifier: Option<String>,
    draw_mode: gl::types::GLenum
}

impl MeshAttributesBuilder {
    pub fn new() -> MeshAttributesBuilder {
        MeshAttributesBuilder { 
            texture_identifier: None,
            font_identifier: None,
            draw_mode: gl::TRIANGLES
        }
    }

    pub fn texture(mut self, texture_identifier: &str) 
        -> MeshAttributesBuilder {
        self.texture_identifier = Some(texture_identifier.into());
        self
    }

    pub fn font(mut self, font_identifier: &str)
        -> MeshAttributesBuilder {
        self.font_identifier = Some(font_identifier.into());
        self
    }

    pub fn draw_mode(mut self, draw_mode: gl::types::GLenum)
        -> MeshAttributesBuilder {
        self.draw_mode = draw_mode;
        self
    }

    pub fn build(self) -> MeshAttributes {
        MeshAttributes {
            texture_identifier: self.texture_identifier,
            font_identifier: self.font_identifier,
            draw_mode: self.draw_mode
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct MeshAttributes {
    texture_identifier: Option<String>,
    font_identifier: Option<String>,
    draw_mode: gl::types::GLenum
}

impl MeshAttributes {
    pub fn defaults() -> MeshAttributes {
        MeshAttributes {
            texture_identifier: None,
            font_identifier: None,
            draw_mode: gl::TRIANGLES
        }
    }

    pub fn texture_identifier(&self) -> &Option<String> {
        &self.texture_identifier
    }

    pub fn font_identifier(&self) -> &Option<String> {
        &self.font_identifier
    }

    pub fn draw_mode(&self) -> gl::types::GLenum {
        self.draw_mode
    }
}

/// Batch of meshes with the same attributes
struct RenderBatch {
    mesh_attributes: MeshAttributes,
    vao: opengl::VertexArrayObject,
    vbo: opengl::BufferObject,
    ebo: opengl::BufferObject,
    vertex_count: usize,
    index_count: usize,
    last_index: usize
}

impl RenderBatch {
    const MAX_BATCH_SIZE: usize = 100000;

    pub fn new(mesh_attributes: MeshAttributes) -> RenderBatch {
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
            mesh_attributes,
            vao,
            vbo,
            ebo,
            vertex_count: 0,
            index_count: 0,
            last_index: 0
        }
    }

    pub fn mesh_attributes(&self) -> MeshAttributes {
        self.mesh_attributes.clone()
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
        opengl::draw_elements(self.mesh_attributes.draw_mode(),
                          self.index_count as gl::types::GLsizei,
                          gl::UNSIGNED_INT,
                          std::ptr::null() as *const gl::types::GLvoid);
    }
}

#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>,
    attributes: MeshAttributes
}

impl Mesh {
    /// Creates a new, empty mesh
    pub fn new(attributes: MeshAttributes) -> Mesh {
        Mesh {
            vertices: vec!(),
            indices: vec!(),
            attributes
        }
    }

    /// Adds vertices to the mesh
    ///
    /// # Examples
    ///
    /// ```
    /// use tuber_graphics_opengl::{Mesh, MeshAttributes, Vertex};
    ///
    /// let mut mesh = Mesh::new(MeshAttributes::defaults());
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
    /// use tuber_graphics_opengl::{Mesh, MeshAttributes};
    ///
    /// let mut mesh = Mesh::new(MeshAttributes::defaults());
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

    pub fn attributes(&self) -> MeshAttributes {
        self.attributes.clone()
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
