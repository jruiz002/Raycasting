use raylib::prelude::*;

pub const MAZE: &[&str] = &[
    "AAAAAAAAAAAA",
    "A     A    A",
    "A BBB A CC A",
    "A B   A C  A",
    "A B BBB CCCC",
    "A   A      A",
    "BBB A DDDD A",
    "A   A    A A",
    "A DDDDD CC A",
    "A       A  A",
    "AAAAAAAAAAEA",
    "AAAAAAAAAAAA",
];

pub fn get_maze_cell(x: f32, y: f32, block_size: i32) -> char {
    let col = (x / block_size as f32) as usize;
    let row = (y / block_size as f32) as usize;
    if row >= MAZE.len() || col >= MAZE[0].len() {
        return '#';
    }
    MAZE[row].chars().nth(col).unwrap_or('#')
}

pub fn is_wall(x: f32, y: f32, block_size: i32) -> bool {
    let cell = get_maze_cell(x, y, block_size);
    cell == '#' || cell == 'A' || cell == 'B' || cell == 'C' || cell == 'D'
}

pub fn wall_color(cell: char) -> Color {
    match cell {
        '#' => Color::new(180, 60, 60, 255),    // Rojo ladrillo
        'A' => Color::new(60, 180, 60, 255),    // Verde
        'B' => Color::new(60, 60, 180, 255),    // Azul
        'C' => Color::new(180, 180, 60, 255),   // Amarillo
        'D' => Color::new(180, 60, 180, 255),   // Magenta
        'E' => Color::new(255, 99, 130, 255),   // Rosa (meta)
        _ => Color::new(120, 120, 120, 255),    // Gris
    }
}

pub fn find_cell(cell: char) -> Option<(usize, usize)> {
    for (row, line) in MAZE.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == cell {
                return Some((row, col));
            }
        }
    }
    None
}