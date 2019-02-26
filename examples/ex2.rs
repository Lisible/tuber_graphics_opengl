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

use sdl2::image::LoadSurface;

use tuber::window::{Window, WindowEvent};
use tuber_window_sdl2::SDLWindow;

use tuber::input::keyboard::Key;

use tuber_graphics_opengl::Vertex;
use tuber_graphics_opengl::shader::{Shader, ShaderProgram};
use tuber_graphics_opengl::opengl;
use tuber_graphics_opengl::offset_of;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let sdl_video_subsystem = sdl_context.video()?;
    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));
    let mut window = SDLWindow::new(&sdl_video_subsystem, sdl_event_pump.clone());
    gl::load_with(|s| sdl_video_subsystem.gl_get_proc_address(s)
                  as *const std::os::raw::c_void);


     
    
    let vertex_shader = Shader::from_source(&CString::new(
            include_str!("shaders/ex2.vert")).unwrap(),
            gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(&CString::new(
            include_str!("shaders/ex2.frag")).unwrap(),
            gl::FRAGMENT_SHADER).unwrap();

    let shader_program = ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    ).unwrap();

    // Texture loading
    let mut texture = opengl::Texture::new();
    unsafe {
        let sdl_surface = sdl2::surface::Surface::from_file("data/64x64.png")?;    
        let mut mode = gl::RGB;
        if sdl_surface.pixel_format_enum().byte_size_per_pixel() == 4 {
            mode = gl::RGBA;
        }
        
        let bytes_per_row = sdl_surface.pitch() as usize;
        let image_height = sdl_surface.height() as usize;
        let mut flipped_image: Vec<u8> = Vec::with_capacity(bytes_per_row * image_height);
        let ptr = sdl_surface.without_lock().unwrap().as_ptr();
        for i in 0..image_height {
            ptr.offset((bytes_per_row * (image_height - i - 1)) as isize).copy_to(flipped_image.as_mut_ptr().offset((i * bytes_per_row) as isize), bytes_per_row);
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
        texture.set_parameter(gl::TEXTURE_MIN_FILTER, TexParam::Int(gl::LINEAR as i32));
        texture.set_parameter(gl::TEXTURE_MAG_FILTER, TexParam::Int(gl::NEAREST as i32));
    }


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
            color: (0.0, 0.0, 1.0),
            texture_coordinates: (0.0, 1.0)
        },
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
    vao.enable_vertex_attrib_array(2);
    vao.vertex_attrib_pointer(
        2,
        2,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vertex>() as gl::types::GLint,
        offset_of!(Vertex, texture_coordinates) as *const gl::types::GLvoid);
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
            texture.bind(gl::TEXTURE_2D);
            vao.bind();
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
