use raylib::prelude::*;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, Source};

const MAZE: &[&str] = &[
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

struct Player {
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
    lives: i32,
    max_lives: i32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            speed: 0.2,
            lives: 3,
            max_lives: 3,
        }
    }

    fn reset_position(&mut self, start_col: usize, start_row: usize, block_size: i32) {
        self.x = (start_col as f32 + 0.5) * block_size as f32;
        self.y = (start_row as f32 + 0.5) * block_size as f32;
        self.angle = 0.0;
    }

    fn lose_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    fn is_alive(&self) -> bool {
        self.lives > 0
    }

    fn reset_lives(&mut self) {
        self.lives = self.max_lives;
    }
}

fn get_maze_cell(x: f32, y: f32, block_size: i32) -> char {
    let col = (x / block_size as f32) as usize;
    let row = (y / block_size as f32) as usize;
    if row >= MAZE.len() || col >= MAZE[0].len() {
        return '#';
    }
    MAZE[row].chars().nth(col).unwrap_or('#')
}

fn is_wall(x: f32, y: f32, block_size: i32) -> bool {
    let cell = get_maze_cell(x, y, block_size);
    cell == '#' || cell == 'A' || cell == 'B' || cell == 'C' || cell == 'D'
}

fn wall_color(cell: char) -> Color {
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

fn find_cell(cell: char) -> Option<(usize, usize)> {
    for (row, line) in MAZE.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == cell {
                return Some((row, col));
            }
        }
    }
    None
}

// Función de raycasting
fn cast_ray(start_x: f32, start_y: f32, angle: f32, block_size: i32) -> (f32, char, bool) {
    let dx = angle.cos();
    let dy = angle.sin();
    
    // Posición actual del rayo
    let mut map_x = (start_x / block_size as f32) as i32;
    let mut map_y = (start_y / block_size as f32) as i32;
    
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;
    
    // Distancia que el rayo debe viajar para ir de un lado x al siguiente, o de un lado y al siguiente
    let delta_dist_x = if dx == 0.0 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy == 0.0 { 1e30 } else { (1.0 / dy).abs() };
    
    let mut hit = false;
    let mut side = false;
    
    // Dirección del paso y distancia inicial al lado
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
    
    // Realizar DDA
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
        
        // Verificar si el rayo golpeó una pared
        if map_x < 0 || map_y < 0 || map_x >= MAZE[0].len() as i32 || map_y >= MAZE.len() as i32 {
            hit = true;
        } else {
            let cell = MAZE[map_y as usize].chars().nth(map_x as usize).unwrap_or('#');
            if cell == '#' || cell == 'A' || cell == 'B' || cell == 'C' || cell == 'D' || cell == 'E' {
                hit = true;
            }
        }
    }
    
    // Calcular distancia
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

fn draw_heart(d: &mut RaylibDrawHandle, x: i32, y: i32, size: i32, color: Color) {
    let radius = size as f32 / 2.0;
    d.draw_circle(x, y, radius, color);
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let block_size = 64; 
    let fov = 1.047; 

    let mouse_sensitivity = 0.003;
    let key_rotation_speed = 0.004;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Laberinto Raycasting 3D - Sistema de Vidas")
        .build();

    rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
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
                rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
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
                rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
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
                rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
            }
            continue;
        }

        let fps = rl.get_fps();

        // Actualizar efectos temporales
        if damage_effect_time > 0.0 {
            damage_effect_time -= rl.get_frame_time() as f64;
        }
        if invulnerability_time > 0.0 {
            invulnerability_time -= rl.get_frame_time() as f64;
        }

        // Movimiento del jugador
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut tried_to_move = false;
        
        if rl.is_key_down(KeyboardKey::KEY_W) {
            dx += player.angle.cos() * player.speed;
            dy += player.angle.sin() * player.speed;
            tried_to_move = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            dx -= player.angle.cos() * player.speed;
            dy -= player.angle.sin() * player.speed;
            tried_to_move = true;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            player.angle -= key_rotation_speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            player.angle += key_rotation_speed;
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
        let center_x = window_width / 2;
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
        let mut d = rl.begin_drawing(&thread);
        
        // Efecto de daño - pantalla roja
        if damage_effect_time > 0.0 {
            let intensity = (damage_effect_time / 0.5 * 100.0) as u8;
            d.clear_background(Color::new(255, intensity, intensity, 255));
        } else {
            d.clear_background(Color::BLACK);
        }

        // Cielo y piso sólidos con colores solicitados
        let sky_color = Color::new(66, 135, 245, 255); 
        let floor_color = Color::new(180, 180, 180, 255); 
        let sky_height = (window_height as f32 * 0.25) as i32;
        for y in 0..sky_height {
            d.draw_line(0, y, window_width, y, sky_color);
        }
        for y in sky_height..window_height {
            d.draw_line(0, y, window_width, y, floor_color);
        }

        // --- EFECTO DE LINTERNA ---
        let light_radius = (window_height as f32 * 0.45) as i32;
        let center_x = (window_width / 2) as i32;
        let center_y = (window_height / 2) as i32;
        for r in (light_radius..(window_width.max(window_height) as i32)).step_by(8) {
            let alpha = ((r - light_radius) as f32 / (window_height as f32 * 0.55)).clamp(0.0, 1.0);
            let darkness = (180.0 * alpha) as u8;
            d.draw_circle(center_x, center_y, r as f32, Color::new(0, 0, 0, darkness));
        }

        // --- RAYCASTING 3D CON META ANIMADA ---
        for x in 0..window_width {
            let ray_angle = player.angle - fov / 2.0 + (x as f32 / window_width as f32) * fov;
            let (distance, wall_type, is_side) = cast_ray(player.x, player.y, ray_angle, block_size);
            let wall_height = (window_height as f32 * block_size as f32 / distance.max(1.0)) as i32;
            let wall_top = (window_height / 2) - wall_height / 2;
            let wall_bottom = wall_top + wall_height;
            let mut wall_color = match wall_type {
                '#' => wall_color(wall_type),    // Rojo ladrillo
                'A' => wall_color(wall_type),    // Verde
                'B' => wall_color(wall_type),    // Azul
                'C' => wall_color(wall_type),    // Amarillo
                'D' => wall_color(wall_type),    // Magenta
                'E' => {
                    // ANIMACIÓN: parpadeo entre rosa y blanco para la meta
                    let t = ((time * 2.0).sin() * 0.5 + 0.5) as f32;
                    Color::new(
                        (255.0 * (1.0 - t) + 255.0 * t) as u8,
                        (99.0 * (1.0 - t) + 255.0 * t) as u8,
                        (130.0 * (1.0 - t) + 255.0 * t) as u8,
                        255
                    )
                },
                _ => Color::WHITE,                         
            };
            if is_side {
                wall_color = Color::new(
                    (wall_color.r as f32 * 0.7) as u8,
                    (wall_color.g as f32 * 0.7) as u8,
                    (wall_color.b as f32 * 0.7) as u8,
                    255,
                );
            }
            let max_distance = block_size as f32 * 15.0;
            let fade = (1.0 - (distance / max_distance)).clamp(0.3, 1.0);
            wall_color = Color::new(
                (wall_color.r as f32 * fade) as u8,
                (wall_color.g as f32 * fade) as u8,
                (wall_color.b as f32 * fade) as u8,
                255,
            );
            
            // Efecto de parpadeo durante invulnerabilidad
            if invulnerability_time > 0.0 && ((current_time * 10.0) as i32 % 2 == 0) {
                wall_color = Color::new(
                    (wall_color.r as f32 * 0.5) as u8,
                    (wall_color.g as f32 * 0.5) as u8,
                    (wall_color.b as f32 * 0.5) as u8,
                    255,
                );
            }
            
            d.draw_line(
                x, 
                wall_top.max(0), 
                x, 
                wall_bottom.min(window_height), 
                wall_color
            );
        }

        // Minimapa mejorado
        let minimap_size = 150;
        let minimap_x = window_width - minimap_size - 10;
        let minimap_y = 10;
        let mini_block = minimap_size / MAZE[0].len() as i32;
        
        d.draw_rectangle(minimap_x - 2, minimap_y - 2, minimap_size + 4, minimap_size + 4, Color::new(66, 135, 245, 200)); // Fondo azul translúcido
        
        // Dibujar celdas del laberinto
        for (row, line) in MAZE.iter().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                let x = minimap_x + (col as i32) * mini_block;
                let y = minimap_y + (row as i32) * mini_block;
                
                let color = match cell {
                    '#' => wall_color(cell),    // Rojo ladrillo
                    'A' => wall_color(cell),    // Verde
                    'B' => wall_color(cell),    // Azul
                    'C' => wall_color(cell),    // Amarillo
                    'D' => wall_color(cell),    // Magenta
                    'E' => wall_color(cell),    // Rosa (meta)
                    _ => Color::WHITE,                     
                };
                
                if cell != ' ' {
                    d.draw_rectangle(x, y, mini_block, mini_block, color);
                }
            }
        }
        
        // Jugador en el minimapa
        let mini_px = minimap_x + (player.x / block_size as f32 * mini_block as f32) as i32;
        let mini_py = minimap_y + (player.y / block_size as f32 * mini_block as f32) as i32;
        
        // Animación pulsante y efecto de invulnerabilidad
        let pulse = (time * 4.0).sin() * 0.3 + 1.0;
        let radius = mini_block as f32 / 6.0 * pulse as f32;
        let player_color = if invulnerability_time > 0.0 && ((current_time * 8.0) as i32 % 2 == 0) {
            Color::RED
        } else {
            Color::YELLOW
        };
        d.draw_circle(mini_px, mini_py, radius, player_color);
        
        // Dirección del jugador
        let dir_length = mini_block as f32 * 0.7;
        let dir_x = mini_px as f32 + player.angle.cos() * dir_length;
        let dir_y = mini_py as f32 + player.angle.sin() * dir_length;
        d.draw_line_ex(
            Vector2 { x: mini_px as f32, y: mini_py as f32 },
            Vector2 { x: dir_x, y: dir_y },
            2.0,
            Color::WHITE
        );

        // Fondo para las vidas
        d.draw_rectangle(5, 5, 350, 150, Color::new(0, 0, 0, 150));
        
        // Título de vidas
        d.draw_text("VIDAS:", 15, 15, 20, Color::WHITE);
        
        // Dibujar corazones
        for i in 0..player.max_lives {
            let heart_x = 90 + i * 35;
            let heart_y = 25;
            let heart_size = 20;
            
            if i < player.lives {
                // Corazón lleno
                draw_heart(&mut d, heart_x, heart_y, heart_size, Color::RED);
            } else {
                // Corazón vacío
                draw_heart(&mut d, heart_x, heart_y, heart_size, Color::new(80, 80, 80, 255));
            }
        }
        
        // Texto de vidas numerico
        let lives_color = match player.lives {
            3 => Color::GREEN,
            2 => Color::YELLOW,
            1 => Color::RED,
            _ => Color::GRAY,
        };
        d.draw_text(&format!("{}/{}", player.lives, player.max_lives), 200, 15, 20, lives_color);
        
        // Estado de invulnerabilidad
        if invulnerability_time > 0.0 {
            let blink = ((current_time * 6.0) as i32 % 2 == 0);
            if blink {
                d.draw_text("INVULNERABLE", 15, 45, 16, Color::GOLD);
            }
        }
        
        // Controles y objetivo
        d.draw_text("WASD: Mover | Mouse: Mirar", 15, 70, 14, Color::WHITE);
        d.draw_text("¡CUIDADO! Pierdes vida al chocar", 15, 90, 12, Color::ORANGE);
        
        // Leyenda de colores de paredes
        d.draw_text("Paredes:", 15, 110, 12, Color::WHITE);
        d.draw_text("A: Verde | B: Azul | C: Amarillo | D: Magenta", 15, 125, 10, Color::LIGHTGRAY);
        
        let fps_color = if fps > 30 { Color::GREEN } else if fps > 15 { Color::YELLOW } else { Color::RED };
        d.draw_text(&format!("FPS: {}", fps), 15, 140, 14, fps_color);
        
        // Advertencia de pocas vidas
        if player.lives == 1 {
            let warning_alpha = ((time * 8.0).sin() * 0.5 + 0.5) as f32;
            let alpha = (255.0 * warning_alpha) as u8;
            d.draw_text("¡ÚLTIMA VIDA!", window_width / 2 - 100, 50, 30, Color::new(255, 0, 0, alpha));
        }
    }
    
    sink.stop();
}