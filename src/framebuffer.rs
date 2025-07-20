use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Color>,
    pub current_color: Color,
    background_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;
        Framebuffer {
            width,
            height,
            pixels: vec![background_color; (width * height) as usize],
            current_color: Color::WHITE,
            background_color,
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }
    
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, path: &str) {
        let mut image = Image::gen_image_color(self.width, self.height, Color::BLANK);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                image.draw_pixel(x, y, self.pixels[index]);
            }
        }
        
        image.export_image(path);
        
        println!("Imagen guardada en '{}'", path);
    }
}