extern crate tuber;
extern crate tuber_window_sdl2;
extern crate tuber_graphics;
extern crate tuber_graphics_opengl;

use tuber::input::Input;
use tuber::input::keyboard::Key;
use tuber::window::Window;
use tuber_window_sdl2::SDLWindow;

use tuber_graphics::Graphics;
use tuber_graphics_opengl::GraphicsOpenGL;

fn main() {
    println!("Hello there!");
    
    let mut window = SDLWindow::new("draw_point", 800u32, 600u32).unwrap();
    window.set_current_graphics_context();
    let graphics = GraphicsOpenGL {};

    'main_loop: loop {
        
        let event = window.poll_event(); 
        match event {
           Input::Close |
           Input::KeyDown(Key::Escape) => break 'main_loop,
           _ => (),
        }

        graphics.clear();
        window.display();
    }
}
