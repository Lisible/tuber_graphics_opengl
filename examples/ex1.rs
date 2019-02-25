/*
* MIT License
*
* Copyright (c) 2018-2019 Clément SIBILLE
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
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

use std::ffi::CString;
use std::rc::Rc;
use std::cell::RefCell; 

use tuber::window::{Window, WindowEvent};
use tuber_window_sdl2::SDLWindow;

use tuber::input::keyboard::Key;

use tuber_graphics_opengl::shader::{Shader, ShaderProgram};
use tuber_graphics_opengl::opengl;
use tuber_graphics_opengl::offset_of;

struct Vertex {
    position: (f32, f32, f32),
    color: (f32, f32, f32)
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video_subsystem = sdl_context.video()?;
    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));
    let mut window = SDLWindow::new(&sdl_video_subsystem, sdl_event_pump.clone());
    gl::load_with(|s| sdl_video_subsystem.gl_get_proc_address(s)
                  as *const std::os::raw::c_void);


     
    
    let vertex_shader = Shader::from_source(&CString::new(
            include_str!("shaders/triangle.vert")).unwrap(),
            gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&CString::new(
            include_str!("shaders/triangle.frag")).unwrap(),
            gl::FRAGMENT_SHADER).unwrap();

    let shader_program = ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    ).unwrap();


    let vertices: Vec<Vertex> = vec![
        Vertex { position: (-0.5, -0.5, 0.0), color: (1.0, 0.0, 0.0) },
        Vertex { position: (0.5, -0.5, 0.0), color: (0.0, 1.0, 0.0) },
        Vertex { position: (0.0, 0.5, 0.0),  color: (0.0, 0.0, 1.0) },
        Vertex { position: (-0.5, 0.5, 0.0),  color: (0.0, 0.0, 1.0) },
        Vertex { position: (0.5, 0.5, 0.0),  color: (0.0, 0.0, 1.0) },
        Vertex { position: (0.0, -0.5, 0.0), color: (0.0, 0.0, 1.0) }
    ];

    let mut vbo = opengl::VBO::new();
    vbo.bind(gl::ARRAY_BUFFER);
    vbo.fill((vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
             vertices.as_ptr() as *const gl::types::GLvoid,
             gl::STATIC_DRAW);
    vbo.unbind();
   
    
    let mut vao = opengl::VAO::new();
    vao.bind();
    vbo.bind(gl::ARRAY_BUFFER);
    vao.enable_vertex_attrib_array(0);
    vao.vertex_attrib_pointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vertex>() as gl::types::GLint,
        std::ptr::null());
    vao.enable_vertex_attrib_array(1);
    vao.vertex_attrib_pointer(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vertex>() as gl::types::GLint,
        offset_of!(Vertex, color) as *const gl::types::GLvoid);
    vbo.unbind(); 
    vao.unbind();

    shader_program.use_program();

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
        unsafe {
            vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                vertices.len() as i32);
        }
        window.display();
    }

    Ok(())
}
