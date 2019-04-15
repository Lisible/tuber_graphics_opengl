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
use tuber_graphics_opengl::{opengl, GLSceneRenderer, font::*};

use tuber::resources::{ResourceLoader, ResourceStore};
use tuber::scene::{SceneGraph, SceneNode, NodeValue};
use tuber::graphics::{scene_renderer::SceneRenderer, Text};

use bmfont_parser::{BMFont, Format};
use nalgebra_glm;

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
        Path::new("data/textured.vert"), 
        gl::VERTEX_SHADER)?;
    let fragment_shader = opengl::Shader::from_file(
        Path::new("data/textured.frag"),
        gl::FRAGMENT_SHADER)?;

    let mut shader_program = opengl::ShaderProgram::from_shaders(
        &[vertex_shader, fragment_shader]
    )?;

    let transform = nalgebra_glm::ortho(0.0, 800.0, 600.0, 0.0, 0.0, 100.0);
    shader_program.use_program();
    shader_program.set_uniform_mat4("transform", transform);
    let texture_store = Rc::new(RefCell::new(GLTextureStore::new()));

    let font_store = Rc::new(RefCell::new(FontStore::new()));
    use tuber::resources::ResourceLoader;
    let mut font_loader = BitmapFontLoader::new();
    font_store.borrow_mut().store("default_font2".into(), font_loader.load("default_font2")?);

    
    let mut scene = SceneGraph::new();
    let text = SceneNode::new("first_text", NodeValue::TextNode(
            Text::new("CAELI 音楽".into(), "default_font2".into())));
    scene.root_mut().add_child(text);

    let mut scene_renderer = GLSceneRenderer::new(texture_store.clone(), font_store.clone());

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

pub struct FontStore {
    fonts: std::collections::HashMap<String, Font>
}

impl FontStore {
    pub fn new() -> FontStore {
        FontStore {
            fonts: std::collections::HashMap::new()
        }
    }
}

impl tuber::resources::ResourceStore<Font> for FontStore {
    fn store(&mut self, resource_file_path: String, value: Font) {
        self.fonts.insert(resource_file_path, value);
    }
    fn remove(&mut self, resource_file_path: &str) {
        self.fonts.remove(resource_file_path);
    }

    fn get(&self, resource_file_path: &str) -> Option<&Font> {
        self.fonts.get(resource_file_path)
    }
    fn get_mut(&mut self, resource_file_path: &str) -> Option<&mut Font> {
        self.fonts.get_mut(resource_file_path)
    }
}

struct BitmapFontLoader {
    texture_loader: GLTextureLoader
}

impl BitmapFontLoader {
    pub fn new() -> BitmapFontLoader {
        BitmapFontLoader {
            texture_loader: GLTextureLoader::new()
        }
    }

    fn load_font(&mut self, font_file_path: &str)
        -> Result<Font, String> {
        let bmfont = match BMFont::from_path(&Format::BMFont, font_file_path) {
            Ok(bmfont) => bmfont,
            Err(_) => panic!("Error loading font")
        };

        let page = bmfont.pages.get(0).unwrap();
        let common_details = bmfont.common_details;


        let (horizontal_scale, vertical_scale) = if let Some(details) = common_details {
            (details.scale_w as f32, details.scale_h as f32)
        } else {
            return Err("Font scale not found".into());
        };

        let texture = self.texture_loader.load_texture(page.image_path.to_str().unwrap())?;
        let mut font = Font::new(texture, horizontal_scale, vertical_scale);

        for (char_id, character) in bmfont.chars {
            let character_metadata = FontCharacter::new(
                character.x as f32,
                character.y as f32,
                character.width as f32,
                character.height as f32,
                character.xoffset as f32,
                character.yoffset as f32,
                character.xadvance as f32
            );

            font.add_character(std::char::from_u32(char_id).unwrap(), character_metadata);
        }

        Ok(font)
    }
}

impl tuber::resources::ResourceLoader<Font> for BitmapFontLoader {
    fn load(&mut self, resource_file_path: &str) -> Result<Font, String> {
        use serde_json::Value;
        use std::{fs::File, io::BufReader, io::Read};

        let mut file_path = String::from("data/");
        file_path += &(resource_file_path.to_owned() + ".jbb");
        let file = File::open(&file_path)
            .expect("Resource file not found");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)
            .expect("Can't read resource file");

        let v: Value = serde_json::from_str(&contents)
            .expect("Can't parse resource file");

        let mut font_file_path = String::from("data/");
        font_file_path += v["font_file"].as_str().unwrap();
        self.load_font(&font_file_path)
    }
}

pub struct GLTextureStore {
    textures: std::collections::HashMap<String, opengl::Texture>
}

impl GLTextureStore {
    pub fn new() -> GLTextureStore {
        GLTextureStore {
            textures: std::collections::HashMap::new()
        }
    }
}

impl tuber::resources::ResourceStore<opengl::Texture> for GLTextureStore {
    fn store(&mut self, resource_file_path: String, value: opengl::Texture) {
        self.textures.insert(resource_file_path, value);
    }
    fn remove(&mut self, resource_file_path: &str) {
        self.textures.remove(resource_file_path);
    }

    fn get(&self, resource_file_path: &str) -> Option<&opengl::Texture> {
        self.textures.get(resource_file_path)
    }
    fn get_mut(&mut self, resource_file_path: &str) -> Option<&mut opengl::Texture> {
        self.textures.get_mut(resource_file_path)
    }
}

struct GLTextureLoader;
impl GLTextureLoader {
    pub fn new() -> GLTextureLoader {
        GLTextureLoader
    }

    pub fn load_texture(&mut self, texture_file_path: &str)
        -> Result<opengl::Texture, String> {
        use sdl2::image::LoadSurface;
        use sdl2::surface::Surface;

        let surface = Surface::from_file(Path::new(&texture_file_path))?;
        let texture = opengl::Texture::new(gl::TEXTURE_2D);


        let mut mode = gl::RGB;
        let height = surface.height() as usize;
        let bytes_per_pixel = surface.pixel_format_enum().byte_size_per_pixel();
        if bytes_per_pixel == 4 {
            mode = gl::RGBA;
        }

        let bytes_per_row = surface.pitch() as usize;
        let mut flipped_image: Vec<u8> = Vec::with_capacity(bytes_per_row * height);
        let image_data_ptr = surface.without_lock().unwrap().as_ptr();
        unsafe {
            for i in 0..height {
                image_data_ptr.offset((bytes_per_row * (height - i - 1)) as isize)
                    .copy_to(flipped_image.as_mut_ptr().offset((i * bytes_per_row) as isize), 
                             bytes_per_row);
            }
        }

        texture.bind();
        texture.set_2d_image_data(0,
                                  mode as gl::types::GLint,
                                  surface.width() as gl::types::GLint,
                                  surface.height() as gl::types::GLint,
                                  0,
                                  mode,
                                  gl::UNSIGNED_BYTE,
                                  flipped_image.as_ptr() as *const gl::types::GLvoid);
        texture.generate_mipmap();
        texture.set_int_parameter(gl::TEXTURE_MIN_FILTER, 
                                  gl::NEAREST as gl::types::GLint);
        texture.set_int_parameter(gl::TEXTURE_MAG_FILTER, 
                                  gl::NEAREST as gl::types::GLint);
        texture.set_int_parameter(gl::TEXTURE_WRAP_S, 
                                  gl::REPEAT as gl::types::GLint);
        texture.set_int_parameter(gl::TEXTURE_WRAP_T, 
                                  gl::REPEAT as gl::types::GLint);

        Ok(texture)
    }
}

impl tuber::resources::ResourceLoader<opengl::Texture> for GLTextureLoader {
    fn load(&mut self, resource_file_path: &str) -> Result<opengl::Texture, String> {
        use serde_json::Value;
        use std::{fs::File, io::BufReader, io::Read};

        let mut file_path = String::from("data/");
        file_path += &(resource_file_path.to_owned() + ".jbb"); 
        let file = File::open(&file_path)
            .expect("Resource file not found"); 
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)
            .expect("Can't read resource file");

        let v: Value = serde_json::from_str(&contents)
            .expect("Can't parse resource file");

        let mut image_file_path = String::from("data/");
        image_file_path += v["image_file"].as_str().unwrap();
        self.load_texture(&image_file_path)
    }
}

