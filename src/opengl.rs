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

//! This modules contains wrappers and utilities for OpenGL 

/// OpenGL buffer object wrapper
pub struct BufferObject {
    identifier: gl::types::GLuint,
    target: gl::types::GLenum,
    is_bound: bool
}

impl BufferObject {
    /// Creates a new buffer object for the given target
    pub fn new(target: gl::types::GLenum) -> BufferObject {
        let mut identifier = 0;
        unsafe { gl::GenBuffers(1, &mut identifier); }
        
        BufferObject {
            identifier,
            target,
            is_bound: false
        }
    }

    /// Binds the buffer to its target
    pub fn bind(&mut self) {
        unsafe { gl::BindBuffer(self.target, self.identifier); }
        self.is_bound = true;
    }

    /// Unbinds the buffer from its target
    pub fn unbind(&mut self) {
        self.panic_if_not_bound();

        unsafe { gl::BindBuffer(self.target, 0); }
        self.is_bound = false;
    }

    /// Sets the buffer's data
    pub fn set_data(&mut self, 
                    size: usize,
                    data: *const gl::types::GLvoid,
                    usage: gl::types::GLenum) {
        self.panic_if_not_bound();

        unsafe { 
            gl::BufferData(self.target,
                                size as gl::types::GLsizeiptr,
                                data,
                                usage);
        }

        // TODO error handling
    }

    pub fn update_data(&mut self,
                       offset: usize,
                       size: usize,
                       data: *const gl::types::GLvoid) {
        self.panic_if_not_bound();

        unsafe {
            gl::BufferSubData(self.target,
                              offset as gl::types::GLintptr,
                              size as gl::types::GLsizeiptr,
                              data);
        }

        // TODO error handling
    }


 
    fn panic_if_not_bound(&self) {
        if !self.is_bound {
            panic!("buffer not bound");
        }
    }

}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.identifier); }
    }
}
