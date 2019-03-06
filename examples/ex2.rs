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

use rand::prelude::*;
use std::ffi::CString;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use tuber::window::{Window, WindowEvent};
use tuber::input::keyboard;

use tuber_window_sdl2::SDLWindow;
use tuber_graphics_opengl::{Vertex, Mesh, Renderer, RenderBatch, RenderBatchConfiguration};

use tuber::resources::{ResourceLoader, ResourceStore};
use tuber_graphics_opengl::{ShaderLoader, ShaderStore, Line, Quad};

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let sdl_event_pump = Rc::new(RefCell::new(sdl_context.event_pump()?));
    let mut window = SDLWindow::new(&video_subsystem, sdl_event_pump.clone());
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) 
                           as *const std::os::raw::c_void);

    let mut renderer = Renderer::new();

    let mut shader_store: Box<ResourceStore<tuber_graphics_opengl::opengl::ShaderProgram>> = Box::new(ShaderStore::new());
    let shader = ShaderLoader::load("shaders/shader")?;
    shader_store.store("shader", shader);

    unsafe { 
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); 
    }

    'main_loop: loop {
        for event in window.poll_event() {
            match event {
                WindowEvent::Close |
                WindowEvent::KeyDown(keyboard::Key::Escape) => break 'main_loop,
                _ => {}
            }
        }

        // Preparing rendering
        renderer.draw_line(Line::new(
                Vertex::with_values((-1.0, -1.0, 0.0), (0.0, 0.0, 1.0), (0.0, 0.0)),
                Vertex::with_values((1.0, 1.0, 0.0), (1.0, 0.0, 1.0), (0.0, 0.0)),
                "shader"
        ));

        let mut rng = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        let mut f = || rng.gen::<f32>() * 1f32 - 0.5f32;
        let mut c = || rng2.gen::<f32>();
        for _ in 0..99 {
            let x = f();
            let y = f();
            let w = f();
            let h = f();
            renderer.draw_quad(Quad::new(
                    Vertex::with_values((x, y, 0.0), (c(), c(), c()), (0.0, 0.0)),
                    Vertex::with_values((x + w, y, 0.0), (c(), c(), c()), (0.0, 0.0)),
                    Vertex::with_values((x + w, y + h, 0.0), (c(), c(), c()), (0.0, 0.0)),
                    Vertex::with_values((x, y + h, 0.0), (c(), c(), c()), (0.0, 0.0)),
                    "shader"
            ));
        }

        // Rendering 
        unsafe{gl::Clear(gl::COLOR_BUFFER_BIT);}
        renderer.render(&shader_store);
        window.display();
    }

    Ok(())
}
