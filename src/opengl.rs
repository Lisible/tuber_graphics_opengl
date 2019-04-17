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
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
* INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

//! This modules contains wrappers and utilities for OpenGL 

use std::ffi::{CString, c_void};

/// Loads OpenGL symbols through a load function
pub fn load_symbols<F>(load_function: F)
where
    F: FnMut(&'static str) -> *const c_void {
    gl::load_with(load_function);
}

/// Wrapper function for glDrawArrays
pub fn draw_arrays(mode: gl::types::GLenum,
                   first: gl::types::GLint,
                   count: gl::types::GLsizei) {
    unsafe { gl::DrawArrays(mode, first, count); }
}
/// Wrapper functino for glDrawElements
pub fn draw_elements(mode: gl::types::GLenum,
                     count: gl::types::GLsizei,
                     data_type: gl::types::GLenum,
                     indices: *const gl::types::GLvoid) {
    unsafe { gl::DrawElements(mode, count, data_type, indices); }
}

/// Sets the viewport
pub fn set_viewport(x: gl::types::GLint, y: gl::types::GLint,
                    width: gl::types::GLint, height: gl::types::GLint) {
    unsafe { gl::Viewport(x, y, width, height); }
}

type Color = (f32, f32, f32, f32);

/// Sets the clear values for the color buffers
pub fn set_clear_color(color: Color) {
    unsafe { gl::ClearColor(color.0, color.1, color.2, color.3); }
}
/// Clear the buffers
pub fn clear(mask: gl::types::GLenum) {
    unsafe { gl::Clear(mask); }
}

/// OpenGL buffer object wrapper
pub struct BufferObject {
    identifier: gl::types::GLuint,
    target: gl::types::GLenum
}

impl BufferObject {
    /// Creates a new buffer object for the given target
    pub fn new(target: gl::types::GLenum) -> BufferObject {
        let mut identifier = 0;
        unsafe { gl::GenBuffers(1, &mut identifier); }
        
        BufferObject {
            identifier,
            target
        }
    }

    /// Creates a new buffer object with a pre-allocated size in bytes
    pub fn with_size(target: gl::types::GLenum, size: usize) -> BufferObject {
        let buffer = BufferObject::new(target);
        buffer.bind();
        buffer.set_data(size, 
                        std::ptr::null() as *const gl::types::GLvoid,
                        gl::DYNAMIC_DRAW);
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

    pub fn unmap(&self) {
        unsafe { gl::UnmapBuffer(self.target); }
    }

    pub fn map_buffer_range(&self,
                            offset: usize,
                            length: usize,
                            access: gl::types::GLbitfield) -> *mut gl::types::GLvoid {
        unsafe { 
            gl::MapBufferRange(self.target,
                               offset as gl::types::GLintptr,
                               length as gl::types::GLsizeiptr,
                               access)
        }
    }

    pub fn map_buffer(&self, access: gl::types::GLenum) -> *mut gl::types::GLvoid {
        unsafe {
            gl::MapBuffer(self.target, access)
        }
    }

    /// Sets the buffer's data
    pub fn set_data(&self, 
                    size: usize,
                    data: *const gl::types::GLvoid,
                    usage: gl::types::GLenum) {
        unsafe { 
            gl::BufferData(self.target,
                                size as gl::types::GLsizeiptr,
                                data,
                                usage);
        }

        // TODO error handling
    }

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

        // TODO error handling
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.identifier); }
    }
}

/// OpenGL vertex array object wrapper
pub struct VertexArrayObject {
    identifier: gl::types::GLuint,
}

impl VertexArrayObject {
    /// Creates a new vertex array object
    pub fn new() -> VertexArrayObject {
        let mut identifier = 0;
        unsafe { gl::GenVertexArrays(1, &mut identifier); }

        VertexArrayObject {
            identifier
        }
    }

    /// Binds the vertex array object
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.identifier); }
    }

    /// Unbinds the vertex array object
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0); }
    }

    /// Enables and sets an attribute of the vertex array object
    pub fn set_attribute(&self,
                         index: usize,
                         size: usize,
                         kind: gl::types::GLenum,
                         normalized: gl::types::GLboolean,
                         stride: usize,
                         pointer: *const gl::types::GLvoid) {
        unsafe {
            gl::EnableVertexAttribArray(index as gl::types::GLuint);
            gl::VertexAttribPointer(index as gl::types::GLuint,
                                    size as gl::types::GLint,
                                    kind,
                                    normalized as gl::types::GLboolean,
                                    stride as gl::types::GLsizei,
                                    pointer);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.identifier); }
    }
}

/// OpenGL shader program wrapper
pub struct ShaderProgram {
    identifier: gl::types::GLuint
}

impl ShaderProgram {
    /// Creates a shader program from a slice of shaders
    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let identifier = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(identifier, shader.identifier()); }
        }

        unsafe { gl::LinkProgram(identifier); }

        let mut success = 1;
        unsafe {
            gl::GetProgramiv(identifier, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut length = 0;
            unsafe {
                gl::GetProgramiv(identifier, gl::INFO_LOG_LENGTH, &mut length);
            }

            let mut buffer = Vec::with_capacity(length as usize + 1);
            buffer.extend([b' '].iter().cycle().take(length as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetProgramInfoLog(identifier,
                                      length,
                                      std::ptr::null_mut(),
                                      error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(identifier, shader.identifier()); }
        }

        Ok(ShaderProgram { identifier })
    }

    /// Uses the shader program
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.identifier); }
    }
}

/// OpenGL shader object wrapper
pub struct Shader {
    identifier: gl::types::GLuint
}

impl Shader {
    pub fn from_file(path: &std::path::Path,
                     kind: gl::types::GLenum) -> Result<Shader, String> {
        let source_code = Shader::read_source_file(&path);
        Shader::from_source(&source_code, kind)
    }

    /// Creates a shader from source code
    pub fn from_source(source_code: &str,
                       kind: gl::types::GLenum) -> Result<Shader, String>{
        let identifier = unsafe { gl::CreateShader(kind) };
        let source_string = CString::new(source_code)
            .expect("Interior nul byte found");
        
        Shader::compile(identifier, source_string)?;

        Ok(Shader { identifier })
    }

    pub fn identifier(&self) -> gl::types::GLuint {
        self.identifier
    }

    /// Reads a shader source file into a string
    fn read_source_file(path: &std::path::Path) -> String {
        use std::fs::File;
        use std::io::Read;

        let mut source_file = File::open(path)
            .expect("Couldn't open shader source file");
        let mut content = String::new();
        source_file.read_to_string(&mut content)
            .expect("Couldn't read shader source file");

        content
    }

    /// Compiles a shader
    fn compile(identifier: gl::types::GLuint,
                   source: CString) -> Result<(), String> {
        unsafe {
            gl::ShaderSource(identifier,
                             1,
                             &source.as_ptr(),
                             std::ptr::null());
            gl::CompileShader(identifier);
        }

        let mut success = 1;
        unsafe {
            gl::GetShaderiv(identifier, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut length = 0;
            unsafe {
                gl::GetShaderiv(identifier, gl::INFO_LOG_LENGTH, &mut length);
            }

            let mut buffer = Vec::with_capacity(length as usize + 1);
            buffer.extend([b' '].iter().cycle().take(length as usize));
            let error = unsafe { CString::from_vec_unchecked(buffer) };

            unsafe {
                gl::GetShaderInfoLog(identifier,
                                     length,
                                     std::ptr::null_mut(),
                                     error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(())
    }
}

/// OpenGL texture wrapper
pub struct Texture {
    identifier: gl::types::GLuint,
    target: gl::types::GLenum
}

impl Texture {
    /// Creates a new texture for the given target
    pub fn new(target: gl::types::GLenum) -> Texture {
        let mut identifier = 0;
        unsafe { gl::GenTextures(1, &mut identifier); }

        Texture {
            identifier,
            target
        }
    }

    /// Sets the image data for a 2D texture
    pub fn set_2d_image_data(&self, 
                             level: gl::types::GLint,
                             internal_format: gl::types::GLint,
                             width: gl::types::GLsizei,
                             height: gl::types::GLsizei,
                             border: gl::types::GLint,
                             format: gl::types::GLenum,
                             data_type: gl::types::GLenum,
                             data: *const gl::types::GLvoid) {
        unsafe {
            gl::TexImage2D(self.target,
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

    /// Generates the texture mipmaps
    pub fn generate_mipmap(&self) {
        unsafe { gl::GenerateMipmap(self.target); }
    }


    /// Sets a integer texture parameter
    pub fn set_int_parameter(&self,
                             parameter_name: gl::types::GLenum,
                             parameter_value: gl::types::GLint) {
        unsafe {
            gl::TexParameteri(self.target, parameter_name, parameter_value);
        }
    }

    /// Binds the texture
    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.target, self.identifier); }
    }

    /// Unbinds the texture
    pub fn unbind(&self) {
        unsafe { gl::BindTexture(self.target, 0); }
    }
}
