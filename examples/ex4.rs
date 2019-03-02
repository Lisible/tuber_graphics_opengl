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

extern crate nalgebra_glm as glm;

use std::ffi::CString;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::image::LoadSurface;

use tuber::window::{Window, WindowEvent};
use tuber_window_sdl2::SDLWindow;

use tuber::input::keyboard::Key;

use tuber_graphics_opengl::*;
use tuber_graphics_opengl::opengl::*;
use tuber_graphics_opengl::shader::*;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video_subsystem = sdl_context.video()?;
    let gl_attr = sdl_video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));
    let mut window = SDLWindow::new(&sdl_video_subsystem, sdl_event_pump.clone());
    gl::load_with(|s| sdl_video_subsystem.gl_get_proc_address(s)
              as *const std::os::raw::c_void);

    let mut shader_program = load_shader();
    let mut texture = load_texture("data/64x64.png")?;

    let vertices: Vec<Vertex> = vec![
        Vertex {
            position: (0.5, 0.5, 0.0),
            color: (1.0, 0.0, 0.0),
            texture_coordinates: (1.0, 1.0)
        },
        Vertex {
            position: (0.5, -0.5, 0.0),
            color: (0.0, 1.0, 0.0),
            texture_coordinates: (1.0, 0.0)
        },
        Vertex {
            position: (-0.5, -0.5, 0.0),
            color: (0.0, 0.0, 1.0),
            texture_coordinates: (0.0, 0.0)
        },
        Vertex {
            position: (-0.5, 0.5, 0.0),
            color: (1.0, 0.0, 0.0),
            texture_coordinates: (0.0, 1.0)
        }
    ];

    let indices: Vec<u32> = vec![
        0, 1, 3,
        1, 2, 3
    ];

    let mut vbo = opengl::BufferObject::new();
    vbo.bind(gl::ARRAY_BUFFER);
    vbo.fill((vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
             vertices.as_ptr() as *const gl::types::GLvoid,
             gl::STATIC_DRAW);
    vbo.unbind();

    let mut ebo = opengl::BufferObject::new();
    ebo.bind(gl::ELEMENT_ARRAY_BUFFER);
    ebo.fill((indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
             indices.as_ptr() as *const gl::types::GLvoid,
             gl::STATIC_DRAW);
    ebo.unbind();

    let mut vao = opengl::VertexArrayObject::new();
    vao.bind();
    vbo.bind(gl::ARRAY_BUFFER);
    ebo.bind(gl::ELEMENT_ARRAY_BUFFER);
    vao.enable_vertex_attrib_array(0);
    vao.vertex_attrib_pointer(0, 3, gl::FLOAT, gl::FALSE,
                              std::mem::size_of::<Vertex>() as gl::types::GLint,
                              std::ptr::null());
    vao.enable_vertex_attrib_array(1);
    vao.vertex_attrib_pointer(1, 3, gl::FLOAT, gl::FALSE,
                              std::mem::size_of::<Vertex>() as gl::types::GLint,
                              std::ptr::null());
    vao.enable_vertex_attrib_array(2);
    vao.vertex_attrib_pointer(2, 2, gl::FLOAT, gl::FALSE,
                              std::mem::size_of::<Vertex>() as gl::types::GLint,
                              std::ptr::null());
    vbo.unbind();
    vao.unbind();

    let rad_ratio = std::f32::consts::PI / 180f32;
    let model: glm::Mat4 = glm::identity();
    let model = glm::rotate_x(&model, -55f32 * rad_ratio);
    let model = glm::scale(&model, &glm::vec3(2f32, 100f32, 1f32));

    let view: glm::Mat4 = glm::identity();
    let view = glm::translate(&view, &glm::vec3(0f32, 0f32, -3f32));

    let projection: glm::Mat4 = glm::perspective(45f32 * rad_ratio, 800f32 / 600f32, 0.1f32, 100.0f32);

    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    'main_loop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                WindowEvent::Close |
                WindowEvent::KeyDown(Key::Escape) => break 'main_loop,
                _ => {}
            }
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        shader_program.use_program();
        shader_program.set_uniform_value("model", 
            UniformValue::MatrixVFloat(4, glm::value_ptr(&model).as_ptr()));
        shader_program.set_uniform_value("view", 
            UniformValue::MatrixVFloat(4, glm::value_ptr(&view).as_ptr()));
        shader_program.set_uniform_value("projection", 
            UniformValue::MatrixVFloat(4, glm::value_ptr(&projection).as_ptr()));

        texture.unbind();
        vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
            gl::UNSIGNED_INT,
                0 as *const gl::types::GLvoid);
        }
        window.display();
    }

    Ok(())
}

fn load_texture(texture_path: &str) -> Result<Texture, String> {
    let mut texture = Texture::new();
    let sdl_surface = sdl2::surface::Surface::from_file(texture_path)?;

    let mut mode = gl::RGB;
    if sdl_surface.pixel_format_enum().byte_size_per_pixel() == 4 {
        mode = gl::RGBA;
    }

    let bytes_per_row = sdl_surface.pitch() as usize;
    let image_height = sdl_surface.height() as usize;
    let mut flipped_image: Vec<u8> = Vec::with_capacity(bytes_per_row * image_height);
    let image_data_ptr = sdl_surface.without_lock().unwrap().as_ptr();
    unsafe { 
        for i in 0..image_height {
            image_data_ptr.offset((bytes_per_row * (image_height - i - 1)) as isize)
                .copy_to(flipped_image.as_mut_ptr().offset((i * bytes_per_row) as isize), bytes_per_row);
        }
    }

    texture.bind(gl::TEXTURE_2D);
    texture.set_image(0,
                      mode as gl::types::GLint,
                      sdl_surface.width() as gl::types::GLsizei,
                      sdl_surface.height() as gl::types::GLsizei,
                      0,
                      mode,
                      gl::UNSIGNED_BYTE,
                      flipped_image.as_ptr() as *const gl::types::GLvoid);
    texture.generate_mipmap();

    use opengl::TextureParameterValue as TexParam;
    texture.set_parameter(gl::TEXTURE_MIN_FILTER, 
                          TexParam::Int(gl::LINEAR as i32));
    texture.set_parameter(gl::TEXTURE_MAG_FILTER,
                          TexParam::Int(gl::NEAREST as i32));
    texture.set_parameter(gl::TEXTURE_WRAP_S, 
                          TexParam::Int(gl::REPEAT as i32));
    texture.set_parameter(gl::TEXTURE_WRAP_S, 
                          TexParam::Int(gl::REPEAT as i32));

    return Ok(texture);
}

fn load_shader() -> ShaderProgram {
    let vertex_shader = Shader::from_source(&CString::new(
            include_str!("shaders/ex4.vert")).unwrap(),
            gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&CString::new(
            include_str!("shaders/ex4.frag")).unwrap(),
            gl::FRAGMENT_SHADER).unwrap();

    ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    ).unwrap()
}
