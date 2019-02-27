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

// The following code is based on the work of Nerijus Arlauskas in his 
// Rust OpenGL lessons, https://github.com/Nercury

use std::ffi::{CString, CStr};
use std::collections::HashMap;

pub struct ShaderProgram {
    id: gl::types::GLuint,
    uniform_locations: HashMap<String, gl::types::GLint>
}

impl ShaderProgram {
    /// Returns the id of the texture
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let id = unsafe { gl::CreateProgram() }; 

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }

        unsafe { gl::LinkProgram(id); }

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); } 

        if success == 0 {
            let mut info_log_len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len); }

            let error = create_whitespace_cstring_with_len(info_log_len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    info_log_len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()); }
        }

        Ok(ShaderProgram { 
            id,
            uniform_locations: HashMap::new()
        })
    }

    /// Returns the uniform location for the uniform
    /// with the given name
    pub fn get_uniform_location(&self, name: &str) -> Option<gl::types::GLint> {
        let location = unsafe {
            gl::GetUniformLocation(self.id(), 
                                   CString::new(name)
                                   .unwrap()
                                   .as_ptr())
        };

        if location == -1 {
            return None;
        }

        Some(location)
    }

    fn cache_uniform_location(&mut self, name: &str) {
        let location = self.get_uniform_location(name);
        if let Some(loc) = location {
            self.uniform_locations.insert(String::from(name), loc);
        }
    }

    pub fn set_uniform_value(&mut self, 
                             name: &str, 
                             value: UniformValue) {
        
        if !self.uniform_locations.contains_key(name) {
            self.cache_uniform_location(name); 
        }

        if let Some(&location) = self.uniform_locations.get(name) {
            unsafe {
                match value {
                    UniformValue::Float(v1) =>
                        gl::Uniform1f(location, v1),
                    UniformValue::Float2(v1, v2) =>
                        gl::Uniform2f(location, v1, v2),
                    UniformValue::Float3(v1, v2, v3) =>
                        gl::Uniform3f(location, v1, v2, v3),
                    UniformValue::Float4(v1, v2, v3, v4) =>
                        gl::Uniform4f(location, v1, v2, v3, v4),

                    UniformValue::Int(v1) =>
                        gl::Uniform1i(location, v1),
                    UniformValue::Int2(v1, v2) =>
                        gl::Uniform2i(location, v1, v2),
                    UniformValue::Int3(v1, v2, v3) =>
                        gl::Uniform3i(location, v1, v2, v3),
                    UniformValue::Int4(v1, v2, v3, v4) =>
                        gl::Uniform4i(location, v1, v2, v3, v4),

                    UniformValue::UInt(v1) =>
                        gl::Uniform1ui(location, v1),
                    UniformValue::UInt2(v1, v2) =>
                        gl::Uniform2ui(location, v1, v2),
                    UniformValue::UInt3(v1, v2, v3) =>
                        gl::Uniform3ui(location, v1, v2, v3),
                    UniformValue::UInt4(v1, v2, v3, v4) =>
                        gl::Uniform4ui(location, v1, v2, v3, v4),

                    UniformValue::VFloat(1, v) =>
                        gl::Uniform1fv(location, 1, v),
                    UniformValue::VFloat(2, v) =>
                        gl::Uniform2fv(location, 1, v),
                    UniformValue::VFloat(3, v) =>
                        gl::Uniform3fv(location, 1, v),
                    UniformValue::VFloat(4, v) =>
                        gl::Uniform4fv(location, 1, v),

                    UniformValue::VInt(1, v) =>
                        gl::Uniform1iv(location, 1, v),
                    UniformValue::VInt(2, v) =>
                        gl::Uniform2iv(location, 1, v),
                    UniformValue::VInt(3, v) =>
                        gl::Uniform3iv(location, 1, v),
                    UniformValue::VInt(4, v) =>
                        gl::Uniform4iv(location, 1, v),

                    UniformValue::VUInt(1, v) =>
                        gl::Uniform1uiv(location, 1, v),
                    UniformValue::VUInt(2, v) =>
                        gl::Uniform2uiv(location, 1, v),
                    UniformValue::VUInt(3, v) =>
                        gl::Uniform3uiv(location, 1, v),
                    UniformValue::VUInt(4, v) =>
                        gl::Uniform4uiv(location, 1, v),

                    UniformValue::MatrixVFloat(2, v) =>
                        gl::UniformMatrix2fv(location, 1, gl::FALSE, v),
                    UniformValue::MatrixVFloat(3, v) =>
                        gl::UniformMatrix3fv(location, 1, gl::FALSE, v),
                    UniformValue::MatrixVFloat(4, v) =>
                        gl::UniformMatrix4fv(location, 1, gl::FALSE, v),
                    
                    _ => panic!("Unknown uniform value")
                }
            }
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

pub enum UniformValue {
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),

    Int(i32),
    Int2(i32, i32),
    Int3(i32, i32, i32),
    Int4(i32, i32, i32, i32),

    UInt(u32),
    UInt2(u32, u32),
    UInt3(u32, u32, u32),
    UInt4(u32, u32, u32, u32),

    VFloat(u32, *const gl::types::GLfloat),
    VInt(u32, *const gl::types::GLint),
    VUInt(u32, *const gl::types::GLuint),

    MatrixVFloat(u32, *const gl::types::GLfloat),
    MatrixVInt(u32, *const gl::types::GLint),
    MatrixVUInt(u32, *const gl::types::GLuint),
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, shader_type: gl::types::GLenum)
        -> Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut info_log_len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut info_log_len);
            }

            let error = create_whitespace_cstring_with_len(info_log_len as usize);
            unsafe {
                gl::GetShaderInfoLog(id,
                                     info_log_len,
                                     std::ptr::null_mut(),
                                     error.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Shader { id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
