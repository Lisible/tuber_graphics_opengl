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

pub struct VertexArrayObject {
    id: gl::types::GLuint,
    bound: bool
}

impl VertexArrayObject {
    /// Creates a new vertex array object
    pub fn new() -> VertexArrayObject {
        let mut id : gl::types::GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id); }
        VertexArrayObject { 
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

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id); }
    }
}

pub struct BufferObject {
    id: gl::types::GLuint,
    target: Option<gl::types::GLenum>,
}

impl BufferObject {
    /// Creates a new buffer object
    pub fn new() -> BufferObject {
        let mut id : gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id); }
        BufferObject { 
            id,
            target: None 
        }
    }

    /// Binds the buffer object to the desired target
    pub fn bind(&mut self, target: gl::types::GLenum) {
        self.target = Some(target);
        unsafe { gl::BindBuffer(target, self.id); }
    }
    /// Unbinds the buffer object
    pub fn unbind(&mut self) {
        if let Some(target) = self.target {
            unsafe { gl::BindBuffer(target, 0); }
            self.target = None;
        }
    }

    /// Fills the buffer object with data
    pub fn fill(&self,
                size: gl::types::GLsizeiptr,
                data: *const gl::types::GLvoid,
                usage: gl::types::GLenum) {
        let target = self.target.expect("BufferObject must be bound to be filled");

        unsafe {
            gl::BufferData(target,
                           size,
                           data,
                           usage);
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id); }
    }
}

pub struct Texture {
    id: gl::types::GLuint,
    target: Option<gl::types::GLenum>,
}

impl Texture {
    /// Creates a new opengl texture object
    pub fn new() -> Texture {
        let mut id : gl::types::GLuint = 0;
        unsafe { gl::GenTextures(1, &mut id); }
        Texture { 
            id,
            target: None
        }
    }

    /// Binds the texture to the given target
    pub fn bind(&mut self, target: gl::types::GLenum) {
        self.target = Some(target);
        unsafe { gl::BindTexture(target, self.id); } 
    }
    /// Unbinds the texture from the current target
    ///
    /// Does nothing if the texture isn't bound
    pub fn unbind(&mut self) {
        if let Some(target) = self.target {
            unsafe { gl::BindTexture(target, 0); }
            self.target = None;
        }
    }
   
    /// Sets the image of the texture
    pub fn set_image(&self, level: gl::types::GLint,
                     internal_format: gl::types::GLint,
                     width: gl::types::GLsizei,
                     height: gl::types::GLsizei,
                     border: gl::types::GLint,
                     format: gl::types::GLenum,
                     data_type: gl::types::GLenum,
                     data: *const gl::types::GLvoid) {
        let target = self.target.expect("Texture must be bound");
        unsafe {
            gl::TexImage2D(target,
                           level,
                           internal_format,
                           width,
                           height,
                           border,
                           format,
                           data_type,
                           data);
        }
    }

    pub fn generate_mipmap(&self) {
        let target = self.target.expect("Texture must be bound");
        unsafe { gl::GenerateMipmap(target); }
    }

    pub fn set_parameter(&self, 
                         parameter_name: gl::types::GLenum, 
                         parameter_value: TextureParameterValue) {
        let target = self.target.expect("Texture must be bound");
        match parameter_value {
            TextureParameterValue::Int(value) => unsafe { 
                gl::TexParameteri(target, parameter_name, value);
            },
            TextureParameterValue::Float(value) => unsafe {
                gl::TexParameterf(target, parameter_name, value);
            },
            TextureParameterValue::Ints(values) => unsafe {
                gl::TexParameteriv(target, parameter_name, values);
            },
            TextureParameterValue::Floats(values) => unsafe {
                gl::TexParameterfv(target, parameter_name, values);
            }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

pub enum TextureParameterValue {
    Int(gl::types::GLint),
    Float(gl::types::GLfloat),
    Ints(*const gl::types::GLint),
    Floats(*const gl::types::GLfloat)
}
