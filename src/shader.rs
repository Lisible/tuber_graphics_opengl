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

pub struct ShaderProgram {
    id: gl::types::GLuint,
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

        Ok(ShaderProgram { id })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
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
