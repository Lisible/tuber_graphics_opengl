/*
* MIT License
*
* Copyright (c) 2018 Clément SIBILLE 
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

pub mod opengl;

/// Represents a vertex in 3D space
pub struct Vertex {
    position: (f32, f32, f32),
    color: (f32, f32, f32),
    texture_coordinates: (f32, f32)
}

impl Vertex {
    /// Creates a vertex with the given values
    pub fn with_values(position: (f32, f32, f32),
                       color: (f32, f32, f32),
                       texture_coordinates: (f32, f32)) -> Vertex {
        Vertex {
            position,
            color,
            texture_coordinates
        }
    }

    pub fn position(&self) -> (f32, f32, f32) {
        self.position
    }
    
    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }

    pub fn texture_coordinates(&self) -> (f32, f32) {
        self.texture_coordinates
    }
}
