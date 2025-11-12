use raylib::prelude::*;

pub struct Framebuffer {
    pub texture: RenderTexture2D,
    pub width: i32,
    pub height: i32,
}

impl Framebuffer {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) -> Self {
        let tex = rl.load_render_texture(thread, width as u32, height as u32).unwrap();
        Self { texture: tex, width, height }
    }

    pub fn begin<'a>(&'a mut self, rl: &'a mut RaylibHandle, thread: &'a RaylibThread) -> RaylibTextureMode<'a, RaylibHandle> {
        rl.begin_texture_mode(thread, &mut self.texture)
    }

    pub fn draw_to_screen(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_rec(
            self.texture.texture(),
            Rectangle::new(0.0, 0.0, self.width as f32, -(self.height as f32)),
            Vector2::new(0.0, 0.0),
            Color::WHITE,
        );
    }
}

pub fn clear(d: &mut RaylibTextureMode<RaylibHandle>, color: Color) {
    d.clear_background(color);
}

pub fn set_pixel(d: &mut RaylibTextureMode<RaylibHandle>, x: i32, y: i32, color: Color) {
    d.draw_pixel(x, y, color);
}