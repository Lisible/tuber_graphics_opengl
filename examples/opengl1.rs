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

use std::cell::RefCell;
use std::rc::Rc;

use tuber::window::{Window, WindowEvent};
use tuber::input::keyboard;

use tuber_window_sdl2::SDLWindow;

use tuber_graphics_opengl::{opengl, Vertex};

fn main() -> Result<(), String> {
    // Setup SDL
    let sdl_context = sdl2::init()?;
    let sdl_video_subsystem = sdl_context.video()?;
    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));

    // Setup SDL GL context
    let sdl_gl_attributes = sdl_video_subsystem.gl_attr();
    sdl_gl_attributes.set_context_profile(sdl2::video::GLProfile::Core);
    sdl_gl_attributes.set_context_version(3, 3);

    // Create window
    let mut window = SDLWindow::new(&sdl_video_subsystem, 
                                    sdl_event_pump.clone());
    // Load gl functions
    opengl::load_symbols(|s| sdl_video_subsystem.gl_get_proc_address(s)
        as *const std::os::raw::c_void);

    
    // Shader loading
    use std::path::Path;
    let vertex_shader = opengl::Shader::from_file(
        Path::new("data/default.vert"), 
        gl::VERTEX_SHADER)?;
    let fragment_shader = opengl::Shader::from_file(
        Path::new("data/default.frag"),
        gl::FRAGMENT_SHADER)?;

    let mut shader_program = opengl::ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    )?;

    shader_program.use_program();
    shader_program.set_uniform_mat4("transform", nalgebra_glm::identity());



    let vertices = [
        Vertex::with_values((0.5, -0.5, 0.0), (1.0, 0.0, 0.0), (0.0, 0.0)),
        Vertex::with_values((-0.5, -0.5, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0)),
        Vertex::with_values((0.0, 0.5, 0.0), (0.0, 0.0, 1.0), (0.0, 0.0)),
    ];

    let vao = opengl::VertexArrayObject::new();
    let vbo = opengl::BufferObject::new(gl::ARRAY_BUFFER);

    vbo.bind();
    vbo.set_data(vertices.len() * std::mem::size_of::<Vertex>(),
                 vertices.as_ptr() as *const gl::types::GLvoid,
                 gl::STATIC_DRAW);
    vbo.unbind();

    vao.bind();
    vbo.bind();
    vao.set_attribute(0, 3, gl::FLOAT, gl::FALSE,
                      std::mem::size_of::<Vertex>(),
                      std::ptr::null() as *const gl::types::GLvoid);
    vao.set_attribute(1, 3, gl::FLOAT, gl::FALSE,
                      std::mem::size_of::<Vertex>(),
                      (3 * std::mem::size_of::<f32>())
                      as *const gl::types::GLvoid);
    vao.set_attribute(2, 2, gl::FLOAT, gl::FALSE,
                      std::mem::size_of::<Vertex>(),
                      (6 * std::mem::size_of::<f32>())
                      as *const gl::types::GLvoid);
    vao.unbind();


    opengl::set_viewport(0, 0, 800, 600);
    opengl::set_clear_color(0.3, 0.3, 0.5);
    'main_loop: loop {
        for event in window.poll_event() {
            match event {
                WindowEvent::Close |
                WindowEvent::KeyDown(keyboard::Key::Escape) => break 'main_loop,
                _ => {}
            }
        }
        
        opengl::clear(gl::COLOR_BUFFER_BIT); 
        
        vao.bind();
        opengl::draw_arrays(gl::TRIANGLES, 0, 
                            vertices.len() as gl::types::GLint);
       
        window.display();
    }

    Ok(())
}

