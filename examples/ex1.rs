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

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use tuber::window::{Window, WindowEvent};
use tuber::input::keyboard;

use tuber_window_sdl2::SDLWindow;
use tuber_graphics_opengl::{opengl, GLSceneRenderer};

use tuber::scene::{SceneGraph, SceneNode, NodeValue};
use tuber::graphics::{scene_renderer::SceneRenderer, Rectangle};

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
    gl::load_with(|s| sdl_video_subsystem.gl_get_proc_address(s)
                  as *const std::os::raw::c_void);

    
    // Shader loading
    let vertex_shader = opengl::Shader::from_file(
        Path::new("data/default.vert"), 
        gl::VERTEX_SHADER)?;
    let fragment_shader = opengl::Shader::from_file(
        Path::new("data/default.frag"),
        gl::FRAGMENT_SHADER)?;

    let shader_program = opengl::ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    )?;

    shader_program.use_program();
   
    let mut scene = SceneGraph::new();
    let rectangle = SceneNode::new("first_rectangle", NodeValue::RectangleNode(
            Rectangle::new(0.25, 0.25, (1.0, 0.0, 0.0, 1.0))));
    scene.root_mut().add_child(rectangle);

    let rectangle2 = SceneNode::new("second_rectangle", NodeValue::RectangleNode(
            Rectangle::new(0.5, 0.5, (1.0, 1.0, 0.0, 1.0))));
    scene.root_mut().add_child(rectangle2);

    let mut scene_renderer = GLSceneRenderer::new();

    opengl::set_viewport(0, 0, 800, 600);
    opengl::set_clear_color((0.3, 0.3, 0.5, 1.0));
    'main_loop: loop {
        for event in window.poll_event() {
            match event {
                WindowEvent::Close |
                WindowEvent::KeyDown(keyboard::Key::Escape) => break 'main_loop,
                _ => {}
            }
        }
        
        opengl::clear(gl::COLOR_BUFFER_BIT); 
        scene_renderer.render_scene(&scene);

        window.display();
    }

    Ok(())
}

