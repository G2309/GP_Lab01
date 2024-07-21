//                          Framebuffer class
//                          Gustavo Cruz
//                          # 22779

use crate::color::Color;

pub struct Framebuffer {
    width: usize,
    height: usize,
    pub buffer: Vec<u32>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width:usize, height:usize, background_color: Color, current_color: Color) -> Self {   
        let buffer = vec![background_color.to_hex(); width * height];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
    }

    pub fn point(&mut self, x:isize, y:isize, color: Color) {
        if x>=0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = color.to_hex();
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(Color::from_hex(self.buffer[index]))
        } else {
            None
        }
    }

    pub fn print(&self){
        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let color = Color::from_hex(self.buffer[index]);
                color.print();
            }
        }
    }

}

