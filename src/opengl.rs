/*
* MIT License
*
* Copyright (c) 2018-2019 Clément SIBILLE
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

pub struct VAO {
    id: gl::types::GLuint,
    bound: bool
}

impl VAO {
    /// Creates a new vertex array object
    pub fn new() -> VAO {
        let mut id : gl::types::GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        VAO { 
            id,
            bound: false
        }
    }

    /// Binds the vertex array object 
    pub fn bind(&mut self) {
        unsafe { gl::BindVertexArray(self.id); }
        self.bound = true;
    }
    /// Unbinds the currently bound vertex array object
    pub fn unbind(&mut self) {
        assert!(self.bound, "VAO must be bound");

        unsafe { gl::BindVertexArray(0); }
        self.bound = false;
    }

    pub fn enable_vertex_attrib_array(&self, index: gl::types::GLuint) {
        assert!(self.bound, "VAO must be bound");
        unsafe { gl::EnableVertexAttribArray(index); }
    }

    pub fn vertex_attrib_pointer(&self,
                                 index: gl::types::GLuint,
                                 size: gl::types::GLint,
                                 value_type: gl::types::GLenum,
                                 normalized: gl::types::GLboolean,
                                 stride: gl::types::GLsizei,
                                 pointer: *const gl::types::GLvoid) {
        assert!(self.bound, "VAO must be bound");
        unsafe {
            gl::VertexAttribPointer(index,
                                    size,
                                    value_type,
                                    normalized,
                                    stride,
                                    pointer);
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}

pub struct VBO {
    id: gl::types::GLuint,
    target: Option<gl::types::GLenum>,
}

impl VBO {
    /// Creates a new vertex buffer object
    pub fn new() -> VBO {
        let mut id : gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        VBO { 
            id,
            target: None 
        }
    }

    /// Binds the vbo to the desired target
    pub fn bind(&mut self, target: gl::types::GLenum) {
        self.target = Some(target);
        unsafe { gl::BindBuffer(target, self.id); }
    }
    /// Unbinds the vbo
    pub fn unbind(&mut self) {
        if let Some(target) = self.target {
            unsafe { gl::BindBuffer(target, 0); }
            self.target = None;
        }
    }

    /// Fills the vbo with data
    pub fn fill(&self,
                size: gl::types::GLsizeiptr,
                data: *const gl::types::GLvoid,
                usage: gl::types::GLenum) {
        let target = self.target.expect("VBO must be bound to be filled");

        unsafe {
            gl::BufferData(target,
                           size,
                           data,
                           usage);
        }
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}
