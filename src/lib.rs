/*
 * MIT License
 *
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
pub mod shader;
pub mod memory_utils;

pub struct Vertex {
    pub position: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub texture_coordinates: (f32, f32)
}

struct Renderer {
    pending_batches: Vec<RenderBatch>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            pending_batches: vec!()
        }
    }
}


struct RenderBatch {
    rendering_mode: gl::types::GLenum,
    vao: opengl::VertexArrayObject,
    vbo: opengl::BufferObject,
    vertices: Vec<Vertex>
}

impl RenderBatch {
    pub fn new() -> RenderBatch {
        RenderBatch {
            rendering_mode: gl::TRIANGLES,
            vao: opengl::VertexArrayObject::new(),
            vbo: opengl::BufferObject::new(),
            vertices: vec!()
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }
}

