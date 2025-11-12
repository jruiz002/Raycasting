use raylib::prelude::*;

pub struct Framebuffer {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
    pub current_color: Color,
    pub background_color: Color,
    texture: Option<Texture2D>,
}

impl Framebuffer {
    // Crear el frame buffer
    pub fn new(width: u32, height: u32) -> Self {
        let total_pixels = (width * height) as usize;
        Self {
            pixels: vec![Color::BLACK; total_pixels],
            width,
            height,
            current_color: Color::WHITE,
            background_color: Color::BLACK,
            texture: None,
        }
    }

    // Limpio el framebuffer
    pub fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = self.background_color;
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = self.current_color;
        }
    }

    pub fn set_pixel_color(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels[index]
        } else {
            Color::BLACK
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_vertical_line(&mut self, x: u32, y_start: i32, y_end: i32) {
        let y_start = y_start.max(0) as u32;
        let y_end = (y_end.min(self.height as i32 - 1)) as u32;
        
        for y in y_start..=y_end {
            self.set_pixel(x, y);
        }
    }

    pub fn draw_rectangle(&mut self, x: u32, y: u32, width: u32, height: u32) {
        for dy in 0..height {
            for dx in 0..width {
                self.set_pixel(x + dx, y + dy);
            }
        }
    }

    pub fn swap_buffers(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Crear una nueva imagen de forma más eficiente
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        
        // Llenar pixel por pixel
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                image.draw_pixel(x as i32, y as i32, pixel);
            }
        }

        // Solo liberar la textura anterior si existe
        if let Some(_old_texture) = self.texture.take() {
            // La textura se libera automáticamente al salir de scope
        }

        // Crear nueva textura desde la imagen
        match rl.load_texture_from_image(thread, &image) {
            Ok(texture) => self.texture = Some(texture),
            Err(_) => {
                // Fallback: mantener la textura anterior si falla la carga
                eprintln!("Error cargando textura del framebuffer");
            }
        }
    }

    pub fn draw_to_screen(&self, d: &mut RaylibDrawHandle) {
        if let Some(ref texture) = self.texture {
            d.draw_texture_rec(
                texture,
                Rectangle::new(0.0, 0.0, self.width as f32, -(self.height as f32)),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );
        }
    }
}