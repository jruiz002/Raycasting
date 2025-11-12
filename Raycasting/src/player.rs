pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub speed: f32,
    pub lives: i32,
    pub max_lives: i32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            speed: 150.0,
            lives: 3,
            max_lives: 3,
        }
    }

    pub fn reset_position(&mut self, start_col: usize, start_row: usize, block_size: i32) {
        self.x = (start_col as f32 + 0.5) * block_size as f32;
        self.y = (start_row as f32 + 0.5) * block_size as f32;
        self.angle = 0.0;
    }

    pub fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }

    pub fn reset_lives(&mut self) {
        self.lives = self.max_lives;
    }
}