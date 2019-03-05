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

pub struct BufferObject {
    identifier: gl::types::GLuint,
    target: gl::types::GLenum,
}

impl BufferObject {
    /// Creates a new, empty buffer object for the given target
    pub fn new(target: gl::types::GLenum) -> BufferObject {
        let mut identifier = 0;
        unsafe { gl::GenBuffers(1, &mut identifier); }

        BufferObject { 
            identifier,
            target
        }
    }

    /// Creates a buffer with the given size
    pub fn with_size<T>(target: gl::types::GLenum,
                        size: usize, 
                        usage: gl::types::GLenum) -> BufferObject {
        let buffer = BufferObject::new(target);
        buffer.bind();
        buffer.set_data(size * std::mem::size_of::<T>(),
                        std::ptr::null(),
                        usage);
        buffer.unbind();

        buffer
    }

    /// Binds the buffer to its target
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target, self.identifier); }
    }

    /// Unbinds the buffer from its target
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.target, 0); }
    }

    /// Sets the data of the buffer
    pub fn set_data(&self,
                    size: usize,
                    data: *const gl::types::GLvoid,
                    usage: gl::types::GLenum) {
        unsafe { 
            gl::BufferData(self.target, size as gl::types::GLsizeiptr, data, usage);
        }
    }

    /// Updates the data of the buffer
    pub fn update_data(&self,
                       offset: usize,
                       size: usize,
                       data: *const gl::types::GLvoid) {
        unsafe {
            gl::BufferSubData(self.target,
                              offset as gl::types::GLintptr,
                              size as gl::types::GLsizeiptr,
                              data);
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.identifier); }
    }
}

pub struct VertexArrayObject {
    identifier: gl::types::GLuint
}

impl VertexArrayObject {
    /// Creates a new vertex array object
    pub fn new() -> VertexArrayObject {
        let mut identifier = 0;
        unsafe { gl::GenVertexArrays(1, &mut identifier); }

        VertexArrayObject { identifier }
    }

    /// Binds the vertex array object
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.identifier); }
    }

    /// Unbinds the vertex array object
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }

    /// Enables the vertex attribute with the given index
    pub fn enable_attribute(&self, index: usize) {
        unsafe { gl::EnableVertexAttribArray(index as gl::types::GLuint); }
    }

    /// Configures the attribute with the given index
    pub fn configure_attribute(&self,
                               index: usize,
                               size: usize,
                               kind: gl::types::GLenum,
                               normalize: gl::types::GLboolean,
                               stride: usize,
                               pointer: usize) {
        unsafe {
            gl::VertexAttribPointer(index as gl::types::GLuint,
                                    size as gl::types::GLint,
                                    kind,
                                    normalize,
                                    stride as gl::types::GLsizei,
                                    pointer as *const gl::types::GLvoid);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.identifier); }
    }
}
