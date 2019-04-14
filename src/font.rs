/*
* MIT License
*
* Copyright (c) 2019 Clément SIBILLE
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

use crate::opengl;
use std::collections::HashMap;

pub struct Font {
    metadata: FontMetadata,
    texture: opengl::Texture
}

impl Font {
    pub fn new(texture: opengl::Texture) -> Font {
        Font {
            metadata: FontMetadata::new(),
            texture
        }
    }

    pub fn add_character(&mut self, character: char,
                         metadata: FontCharacterMetadata) {
        self.metadata.add_character(character, metadata);
    }

    pub fn metadata(&self) -> &FontMetadata {
        &self.metadata
    }

    pub fn bind_texture(&self) {
        self.texture.bind();
    }

    pub fn unbind_texture(&self) {
        self.texture.unbind();
    }
}

pub struct FontMetadata {
    characters: HashMap<char, FontCharacterMetadata>,
}

impl FontMetadata {
    pub fn new() -> FontMetadata {
        FontMetadata {
            characters: HashMap::new()
        }
    }

    pub fn add_character(&mut self, character: char,
                         metadata: FontCharacterMetadata) {
        self.characters.insert(character, metadata);
    }

    pub fn character(&self, character: char)  -> Option<&FontCharacterMetadata> {
        self.characters.get(&character)
    }
}

pub struct FontCharacterMetadata {
    x_coordinate: f32,
    y_coordinate: f32,
    width: f32,
    height: f32,
    x_offset: f32,
    y_offset: f32,
    x_advance: f32
}

impl FontCharacterMetadata {
    pub fn new(x_coordinate: f32, y_coordinate: f32, width: f32, height: f32,
               x_offset: f32, y_offset: f32, x_advance: f32)
               -> FontCharacterMetadata {
        FontCharacterMetadata {
            x_coordinate,
            y_coordinate,
            width,
            height,
            x_offset,
            y_offset,
            x_advance,
        }
    }

    pub fn x_coordinate(&self) -> f32 {
        self.x_coordinate
    }
    pub fn y_coordinate(&self) -> f32 {
        self.y_coordinate
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn x_offset(&self) -> f32 {
        self.x_offset
    }
    pub fn y_offset(&self) -> f32 {
        self.y_offset
    }

    pub fn x_advance(&self) -> f32 {
        self.x_advance
    }
}