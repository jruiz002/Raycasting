use raylib::prelude::*;
use crate::maze::{MAZE, wall_color};
use crate::player::Player;
use crate::framebuffer::Framebuffer;

pub fn render_scene(
    framebuffer: &mut Framebuffer,
    player: &Player,
    fov: f32,
    block_size: i32,
    time: f64,
    invulnerability_time: f64,
    screen_width: i32,
    screen_height: i32,
) {
    // Limpiar framebuffer
    framebuffer.clear();

    // Renderizar cielo y piso
    let sky_color = Color::new(66, 135, 245, 255);
    let floor_color = Color::new(180, 180, 180, 255);
    let sky_height = (screen_height as f32 * 0.25) as i32;
    
    // Dibujar cielo
    for y in 0..sky_height {
        for x in 0..screen_width {
            framebuffer.set_pixel_color(x as u32, y as u32, sky_color);
        }
    }
    
    // Dibujar piso
    for y in sky_height..screen_height {
        for x in 0..screen_width {
            framebuffer.set_pixel_color(x as u32, y as u32, floor_color);
        }
    }

    // Raycasting vertical por columnas
    for x in 0..screen_width {
        let ray_angle = player.angle - fov / 2.0 + (x as f32 / screen_width as f32) * fov;
        let (distance, wall_type, is_side) = cast_ray(player.x, player.y, ray_angle, block_size);
        let wall_height = (screen_height as f32 * block_size as f32 / distance.max(1.0)) as i32;
        let wall_top = (screen_height / 2) - wall_height / 2;
        let wall_bottom = wall_top + wall_height;

        let mut col = match wall_type {
            '#' => wall_color(wall_type),
            'A' => wall_color(wall_type),
            'B' => wall_color(wall_type),
            'C' => wall_color(wall_type),
            'D' => wall_color(wall_type),
            'E' => {
                let t = ((time * 2.0).sin() * 0.5 + 0.5) as f32;
                Color::new(
                    (255.0 * (1.0 - t) + 255.0 * t) as u8,
                    (99.0 * (1.0 - t) + 255.0 * t) as u8,
                    (130.0 * (1.0 - t) + 255.0 * t) as u8,
                    255,
                )
            }
            _ => Color::WHITE,
        };
        
        if is_side {
            col = Color::new(
                (col.r as f32 * 0.7) as u8,
                (col.g as f32 * 0.7) as u8,
                (col.b as f32 * 0.7) as u8,
                255,
            );
        }

        let max_distance = block_size as f32 * 15.0;
        let fade = (1.0 - (distance / max_distance)).clamp(0.3, 1.0);
        col = Color::new(
            (col.r as f32 * fade) as u8,
            (col.g as f32 * fade) as u8,
            (col.b as f32 * fade) as u8,
            255,
        );

        if invulnerability_time > 0.0 && ((time * 10.0) as i32 % 2 == 0) {
            col = Color::new(
                (col.r as f32 * 0.5) as u8,
                (col.g as f32 * 0.5) as u8,
                (col.b as f32 * 0.5) as u8,
                255,
            );
        }

        // Dibujar línea vertical de la pared
        framebuffer.set_current_color(col);
        framebuffer.draw_vertical_line(x as u32, wall_top.max(0), wall_bottom.min(screen_height));
    }
}

fn cast_ray(start_x: f32, start_y: f32, angle: f32, block_size: i32) -> (f32, char, bool) {
    let dx = angle.cos();
    let dy = angle.sin();

    let mut map_x = (start_x / block_size as f32) as i32;
    let mut map_y = (start_y / block_size as f32) as i32;

    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    let delta_dist_x = if dx == 0.0 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy == 0.0 { 1e30 } else { (1.0 / dy).abs() };

    let mut hit = false;
    let mut side = false;

    let step_x: i32;
    let step_y: i32;

    if dx < 0.0 {
        step_x = -1;
        side_dist_x = (start_x / block_size as f32 - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - start_x / block_size as f32) * delta_dist_x;
    }

    if dy < 0.0 {
        step_y = -1;
        side_dist_y = (start_y / block_size as f32 - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - start_y / block_size as f32) * delta_dist_y;
    }

    while !hit {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = false;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = true;
        }

        if map_x < 0 || map_y < 0 || map_x >= MAZE[0].len() as i32 || map_y >= MAZE.len() as i32 {
            hit = true;
        } else {
            let cell = MAZE[map_y as usize].chars().nth(map_x as usize).unwrap_or('#');
            if matches!(cell, '#' | 'A' | 'B' | 'C' | 'D' | 'E') {
                hit = true;
            }
        }
    }

    let perp_wall_dist = if !side {
        (map_x as f32 - start_x / block_size as f32 + (1.0 - step_x as f32) / 2.0) / dx
    } else {
        (map_y as f32 - start_y / block_size as f32 + (1.0 - step_y as f32) / 2.0) / dy
    };

    let distance = perp_wall_dist * block_size as f32;

    let wall_type = if map_x < 0 || map_y < 0 || map_x >= MAZE[0].len() as i32 || map_y >= MAZE.len() as i32 {
        '#'
    } else {
        MAZE[map_y as usize].chars().nth(map_x as usize).unwrap_or('#')
    };

    (distance, wall_type, side)
}