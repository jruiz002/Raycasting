use raylib::prelude::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, Source};

mod player;
mod maze;
mod framebuffer;
mod renderer;
mod ui;

use player::Player;
use maze::{MAZE, get_maze_cell, is_wall, wall_color, find_cell};
use framebuffer::{Framebuffer, clear as fb_clear};
use renderer::render_scene;
use ui::{draw_minimap, draw_hud};



// cast_ray ahora vive en renderer.rs

// draw_heart ahora vive en ui.rs

fn main() {
    const SCREEN_WIDTH: i32 = 1280;
    const SCREEN_HEIGHT: i32 = 720;
    let block_size = 64; 
    let fov = 1.047; 

    let mouse_sensitivity = 0.003;
    let key_rotation_speed = 0.24; // rad/s para teclas A/D

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Laberinto Raycasting 3D - Sistema de Vidas")
        .build();

    let mut framebuffer = Framebuffer::new(&mut rl, &thread, SCREEN_WIDTH, SCREEN_HEIGHT);

    rl.set_mouse_position((SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
    rl.disable_cursor();

    // Encuentra el punto inicial y final
    let (start_row, start_col) = find_cell(' ').unwrap_or((1, 1));
    let (_end_row, _end_col) = find_cell('E').unwrap_or((10, 10));
    let mut player = Player::new(
        (start_col as f32 + 0.5) * block_size as f32,
        (start_row as f32 + 0.5) * block_size as f32
    );
    let mut show_instructions = true;
    let mut show_success = false;
    let mut show_game_over = false;
    let mut damage_effect_time = 0.0f64;
    let mut invulnerability_time = 0.0f64;

    // Música de fondo
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let file = File::open("assets/laberinto.mp3").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
    sink.append(source);
    sink.play();

    // Efectos de sonido
    let (_fx_stream, fx_stream_handle) = OutputStream::try_default().unwrap();
    let fx_sink = Sink::try_new(&fx_stream_handle).unwrap();
    let bump_file_path = "assets/bump.wav";
    let mut last_bump_time = 0.0f64;

    while !rl.window_should_close() {
        let current_time = rl.get_time();

        // Pantalla de instrucciones
        if show_instructions {
            let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::DARKBLUE);
            d.draw_text("LABERINTO 3D RAYCASTING", 80, 80, 40, Color::YELLOW);
            d.draw_text("Controles:", 120, 160, 30, Color::WHITE);
            d.draw_text("- W/S: Avanzar / Retroceder", 140, 200, 24, Color::LIGHTGRAY);
            d.draw_text("- A/D: Girar izquierda / derecha", 140, 230, 24, Color::LIGHTGRAY);
            d.draw_text("- Mouse: Mirar alrededor", 140, 260, 24, Color::LIGHTGRAY);
            d.draw_text("- ESC: Salir", 140, 290, 24, Color::LIGHTGRAY);
            d.draw_text("Sistema de Vidas", 120, 340, 28, Color::RED);
            d.draw_text("- Tienes 3 vidas", 140, 370, 20, Color::LIGHTGRAY);
            d.draw_text("- Pierdes 1 vida al chocar con paredes", 140, 390, 20, Color::LIGHTGRAY);
            d.draw_text("- Sin vidas = Game Over", 140, 410, 20, Color::LIGHTGRAY);
            d.draw_text("NUEVO: Paredes con colores diferentes", 80, 430, 18, Color::ORANGE);
            d.draw_text("Objetivo: Encuentra la salida marcada", 80, 450, 18, Color::LIME);
            d.draw_text("Presiona ENTER para comenzar", 120, 500, 30, Color::GREEN);
            drop(d);
            if enter_pressed {
                show_instructions = false;
                player.reset_lives();
                player.reset_position(start_col, start_row, block_size);
                rl.set_mouse_position((SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
            }
            continue;
        }

        // Pantalla de Game Over
        if show_game_over {
            let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::MAROON);
            d.draw_text("GAME OVER", 200, 180, 60, Color::RED);
            d.draw_text("¡Te quedaste sin vidas!", 180, 260, 30, Color::WHITE);
            d.draw_text("¡Fuiste demasiado descuidado!", 150, 300, 24, Color::LIGHTGRAY);
            d.draw_text("Presiona ENTER para reiniciar", 140, 380, 30, Color::YELLOW);
            drop(d);
            if enter_pressed {
                player.reset_lives();
                player.reset_position(start_col, start_row, block_size);
                show_game_over = false;
                show_instructions = true;
                damage_effect_time = 0.0;
                invulnerability_time = 0.0;
                rl.set_mouse_position((SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
            }
            continue;
        }

        // Pantalla de éxito
        if show_success {
            let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::DARKBLUE);
            d.draw_text("¡FELICIDADES!", 200, 180, 50, Color::YELLOW);
            d.draw_text("¡Has completado el laberinto!", 150, 250, 30, Color::LIME);
            d.draw_text(&format!("Vidas restantes: {}", player.lives), 200, 300, 24, Color::WHITE);
            let score_bonus = player.lives * 100;
            d.draw_text(&format!("Bonus por vidas: {} puntos", score_bonus), 170, 330, 20, Color::GOLD);
            d.draw_text("Presiona ENTER para reiniciar", 140, 400, 30, Color::WHITE);
            drop(d);
            if enter_pressed {
                player.reset_lives();
                player.reset_position(start_col, start_row, block_size);
                show_success = false;
                show_instructions = true;
                damage_effect_time = 0.0;
                invulnerability_time = 0.0;
                rl.set_mouse_position((SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0));
            }
            continue;
        }

        let fps = rl.get_fps();

        // Delta de tiempo
        let dt = rl.get_frame_time();

        // Actualizar efectos temporales
        if damage_effect_time > 0.0 {
            damage_effect_time -= dt as f64;
        }
        if invulnerability_time > 0.0 {
            invulnerability_time -= dt as f64;
        }

        // Movimiento del jugador
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut tried_to_move = false;
        
        if rl.is_key_down(KeyboardKey::KEY_W) {
            dx += player.angle.cos() * player.speed * dt;
            dy += player.angle.sin() * player.speed * dt;
            tried_to_move = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dx -= player.angle.cos() * player.speed * dt;
            dy -= player.angle.sin() * player.speed * dt;
            tried_to_move = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.angle -= key_rotation_speed * dt;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.angle += key_rotation_speed * dt;
        }

        // Colisiones y sistema de vidas
        let mut collided = false;
        let next_x = player.x + dx;
        let next_y = player.y + dy;
        
        if !is_wall(next_x, player.y, block_size) {
            player.x = next_x;
        } else if tried_to_move && dx != 0.0 {
            collided = true;
        }
        
        if !is_wall(player.x, next_y, block_size) {
            player.y = next_y;
        } else if tried_to_move && dy != 0.0 {
            collided = true;
        }

        // Si hubo colisión y no está en período de invulnerabilidad
        if collided && invulnerability_time <= 0.0 {
            let now = rl.get_time();
            if now - last_bump_time > 0.1 {
                // Reproducir sonido
                match File::open(bump_file_path) {
                    Ok(file) => match Decoder::new(BufReader::new(file)) {
                        Ok(source) => {
                            fx_sink.append(source);
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
                last_bump_time = now;
                
                // Perder vida y activar efectos
                player.lose_life();
                damage_effect_time = 0.5; // Efecto de daño por 0.5 segundos
                invulnerability_time = 1.0; // Invulnerabilidad por 1 segundo
                
                // Verificar si se acabaron las vidas
                if !player.is_alive() {
                    show_game_over = true;
                    continue;
                }
            }
        }

        // Rotación con mouse
        let mouse_x = rl.get_mouse_x();
        let center_x = SCREEN_WIDTH / 2;
        let delta_x = mouse_x - center_x;
        player.angle += delta_x as f32 * mouse_sensitivity;
        rl.set_mouse_position((center_x as f32, rl.get_mouse_y() as f32));

        // Detectar llegada a la meta
        let player_cell = get_maze_cell(player.x, player.y, block_size);
        if player_cell == 'E' {
            show_success = true;
            continue;
        }

        let time = rl.get_time();

        let mut d_texture = framebuffer.begin(&mut rl, &thread);

        // Efecto de daño - pantalla roja
        if damage_effect_time > 0.0 {
            let intensity = (damage_effect_time / 0.5 * 100.0) as u8;
            d_texture.clear_background(Color::new(255, intensity, intensity, 255));
        } else {
            d_texture.clear_background(Color::BLACK);
        }

        // Escena 3D en la textura
        render_scene(&mut d_texture, &player, fov, block_size, time, invulnerability_time, SCREEN_WIDTH, SCREEN_HEIGHT);

        drop(d_texture);

        let mut d = rl.begin_drawing(&thread);
        framebuffer.draw_to_screen(&mut d);

        // --- EFECTO DE LINTERNA (usando anillos para no cubrir el centro) ---
        let light_radius = (SCREEN_HEIGHT as f32 * 0.45) as i32;
        let center = Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);
        let max_r = SCREEN_WIDTH.max(SCREEN_HEIGHT) as i32;
        let step = 12;
        for r in (light_radius..max_r).step_by(step as usize) {
            let alpha = ((r - light_radius) as f32 / (SCREEN_HEIGHT as f32 * 0.55)).clamp(0.0, 1.0);
            let darkness = (180.0 * alpha) as u8;
            d.draw_ring(center, r as f32, (r + step) as f32, 0.0, 360.0, 64, Color::new(0, 0, 0, darkness));
        }

        // Minimapa
        draw_minimap(&mut d, &player, block_size, SCREEN_WIDTH, time, invulnerability_time);

        // HUD
        draw_hud(&mut d, &player, invulnerability_time, current_time, fps, SCREEN_WIDTH);
    }
    
    sink.stop();
}