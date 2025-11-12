use raylib::prelude::*;
use crate::maze::{MAZE, wall_color};
use crate::player::Player;
use crate::framebuffer::Framebuffer;

pub fn draw_minimap_to_framebuffer(
    framebuffer: &mut Framebuffer,
    player: &Player,
    block_size: i32,
    screen_width: i32,
    time: f64,
    invulnerability_time: f64,
) {
    let minimap_size = 150;
    let minimap_x = screen_width - minimap_size - 10;
    let minimap_y = 10;
    let mini_block = minimap_size / MAZE[0].len() as i32;

    // Dibujar fondo del minimapa
    let bg_color = Color::new(66, 135, 245, 200);
    framebuffer.set_current_color(bg_color);
    framebuffer.draw_rectangle(
        (minimap_x - 2) as u32,
        (minimap_y - 2) as u32,
        (minimap_size + 4) as u32,
        (minimap_size + 4) as u32,
    );

    // Dibujar celdas del laberinto
    for (row, line) in MAZE.iter().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let x = minimap_x + (col as i32) * mini_block;
            let y = minimap_y + (row as i32) * mini_block;

            let color = match cell {
                '#' => wall_color(cell),
                'A' => wall_color(cell),
                'B' => wall_color(cell),
                'C' => wall_color(cell),
                'D' => wall_color(cell),
                'E' => wall_color(cell),
                _ => continue, // Espacios vacíos no se dibujan
            };

            framebuffer.set_current_color(color);
            framebuffer.draw_rectangle(x as u32, y as u32, mini_block as u32, mini_block as u32);
        }
    }

    // Dibujar jugador
    let mini_px = minimap_x + (player.x / block_size as f32 * mini_block as f32) as i32;
    let mini_py = minimap_y + (player.y / block_size as f32 * mini_block as f32) as i32;

    let pulse = (time * 4.0).sin() * 0.3 + 1.0;
    let radius = (mini_block as f32 / 6.0 * pulse as f32) as i32;
    let player_color = if invulnerability_time > 0.0 && ((time * 8.0) as i32 % 2 == 0) {
        Color::RED
    } else {
        Color::YELLOW
    };

    // Dibujar círculo del jugador (aproximado con rectángulo)
    framebuffer.set_current_color(player_color);
    let player_size = radius.max(2);
    framebuffer.draw_rectangle(
        (mini_px - player_size / 2) as u32,
        (mini_py - player_size / 2) as u32,
        player_size as u32,
        player_size as u32,
    );

    // Dibujar dirección del jugador
    let dir_length = mini_block as f32 * 0.7;
    let dir_x = mini_px + (player.angle.cos() * dir_length) as i32;
    let dir_y = mini_py + (player.angle.sin() * dir_length) as i32;
    
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_line(mini_px, mini_py, dir_x, dir_y);
}

pub fn draw_minimap(d: &mut RaylibDrawHandle, player: &Player, block_size: i32, screen_width: i32, time: f64, invulnerability_time: f64) {
    let minimap_size = 150;
    let minimap_x = screen_width - minimap_size - 10;
    let minimap_y = 10;
    let mini_block = minimap_size / MAZE[0].len() as i32;

    d.draw_rectangle(minimap_x - 2, minimap_y - 2, minimap_size + 4, minimap_size + 4, Color::new(66, 135, 245, 200));

    for (row, line) in MAZE.iter().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            let x = minimap_x + (col as i32) * mini_block;
            let y = minimap_y + (row as i32) * mini_block;

            let color = match cell {
                '#' => wall_color(cell),
                'A' => wall_color(cell),
                'B' => wall_color(cell),
                'C' => wall_color(cell),
                'D' => wall_color(cell),
                'E' => wall_color(cell),
                _ => Color::WHITE,
            };

            if cell != ' ' {
                d.draw_rectangle(x, y, mini_block, mini_block, color);
            }
        }
    }

    let mini_px = minimap_x + (player.x / block_size as f32 * mini_block as f32) as i32;
    let mini_py = minimap_y + (player.y / block_size as f32 * mini_block as f32) as i32;

    let pulse = (time * 4.0).sin() * 0.3 + 1.0;
    let radius = mini_block as f32 / 6.0 * pulse as f32;
    let player_color = if invulnerability_time > 0.0 && ((time * 8.0) as i32 % 2 == 0) {
        Color::RED
    } else {
        Color::YELLOW
    };
    d.draw_circle(mini_px, mini_py, radius, player_color);

    let dir_length = mini_block as f32 * 0.7;
    let dir_x = mini_px as f32 + player.angle.cos() * dir_length;
    let dir_y = mini_py as f32 + player.angle.sin() * dir_length;
    d.draw_line_ex(
        Vector2 { x: mini_px as f32, y: mini_py as f32 },
        Vector2 { x: dir_x, y: dir_y },
        2.0,
        Color::WHITE,
    );
}

pub fn draw_hud(d: &mut RaylibDrawHandle, player: &Player, invulnerability_time: f64, current_time: f64, fps: u32, screen_width: i32) {
    d.draw_rectangle(5, 5, 350, 150, Color::new(0, 0, 0, 150));
    d.draw_text("VIDAS:", 15, 15, 20, Color::WHITE);

    for i in 0..player.max_lives {
        let heart_x = 90 + i * 35;
        let heart_y = 25;
        let heart_size = 20;
        if i < player.lives {
            draw_heart(d, heart_x, heart_y, heart_size, Color::RED);
        } else {
            draw_heart(d, heart_x, heart_y, heart_size, Color::new(80, 80, 80, 255));
        }
    }

    let lives_color = match player.lives {
        3 => Color::GREEN,
        2 => Color::YELLOW,
        1 => Color::RED,
        _ => Color::GRAY,
    };
    d.draw_text(&format!("{}/{}", player.lives, player.max_lives), 200, 15, 20, lives_color);

    if invulnerability_time > 0.0 {
        let blink = (current_time * 6.0) as i32 % 2 == 0;
        if blink {
            d.draw_text("INVULNERABLE", 15, 45, 16, Color::GOLD);
        }
    }

    d.draw_text("WASD: Mover | Mouse: Mirar", 15, 70, 14, Color::WHITE);
    d.draw_text("¡CUIDADO! Pierdes vida al chocar", 15, 90, 12, Color::ORANGE);
    d.draw_text("Paredes:", 15, 110, 12, Color::WHITE);
    d.draw_text("A: Verde | B: Azul | C: Amarillo | D: Magenta", 15, 125, 10, Color::LIGHTGRAY);

    let fps_color = if fps > 30 { Color::GREEN } else if fps > 15 { Color::YELLOW } else { Color::RED };
    d.draw_text(&format!("FPS: {}", fps), 15, 145, 16, fps_color);

    if player.lives == 1 {
        let alpha = ((current_time * 3.0).sin() * 0.5 + 0.5) * 255.0;
        let alpha = alpha.clamp(0.0, 255.0) as u8;
        d.draw_text("¡ÚLTIMA VIDA!", screen_width / 2 - 100, 50, 30, Color::new(255, 0, 0, alpha));
    }
}

pub fn draw_heart(d: &mut RaylibDrawHandle, x: i32, y: i32, size: i32, color: Color) {
    let radius = size as f32 / 2.0;
    d.draw_circle(x, y, radius, color);
}