use raylib::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};

const MAZE: &[&str] = &[
    "############",
    "#     #    #",
    "# ### # ## #",
    "# #   # #  #",
    "# # ### ####",
    "#   #      #",
    "### # #### #",
    "#   #    # #",
    "# ##### ## #",
    "#       #  #",
    "##########E#",
    "############",
];

struct Player {
    x: f32,
    y: f32,
    angle: f32, // en radianes
    speed: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            angle: 0.0,
            speed: 3.0, // velocidad aumentada
        }
    }
}

fn is_wall(x: f32, y: f32, block_size: i32) -> bool {
    let col = (x / block_size as f32) as usize;
    let row = (y / block_size as f32) as usize;
    if row >= MAZE.len() || col >= MAZE[0].len() {
        return true;
    }
    MAZE[row].chars().nth(col) == Some('#')
}

fn wall_color(cell: char) -> Color {
    match cell {
        '#' => Color::RED,
        'A' => Color::GREEN,
        'B' => Color::BLUE,
        'C' => Color::PURPLE,
        'E' => Color::LIME,
        _ => Color::GRAY,
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

fn main() {
    let window_width = 800;
    let window_height = 800;
    let block_size = window_width / MAZE[0].len() as i32;
    let fov = 1.2; // FOV más natural (~69 grados)
    let num_rays = window_width / 2; // Usar la mitad izquierda para la vista 3D

    let mouse_sensitivity = 0.002; // Sensibilidad baja para el mouse
    let key_rotation_speed = 0.025; // Sensibilidad baja para el teclado
    let bump_file_path = "assets/bump.wav";

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Laberinto Raycasting")
        .build();

    rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
    rl.disable_cursor();

    // Encuentra el punto inicial (primer espacio vacío) y el final ('E')
    let (start_row, start_col) = find_cell(' ').unwrap_or((1, 1));
    let (end_row, end_col) = find_cell('E').unwrap_or((10, 10));
    let mut player = Player::new((start_col as f32 + 0.5) * block_size as f32, (start_row as f32 + 0.5) * block_size as f32);
    let mut show_instructions = true;
    let mut show_success = false;

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

    while !rl.window_should_close() {
        // Pantalla de bienvenida/instrucciones
        if show_instructions {
            // Leer entrada antes de begin_drawing
            let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::DARKBLUE);
            d.draw_text("LABERINTO RAYCASTING", 120, 100, 40, Color::YELLOW);
            d.draw_text("Controles:", 120, 180, 30, Color::WHITE);
            d.draw_text("- W/S: Avanzar / Retroceder", 140, 220, 24, Color::LIGHTGRAY);
            d.draw_text("- A/D: Girar a la izquierda / derecha", 140, 250, 24, Color::LIGHTGRAY);
            d.draw_text("- Mouse: Girar la cámara horizontalmente", 140, 280, 24, Color::LIGHTGRAY);
            d.draw_text("- ESC: Salir", 140, 310, 24, Color::LIGHTGRAY);
            d.draw_text("- Mueve el mouse para mirar alrededor", 140, 340, 24, Color::LIGHTGRAY);
            d.draw_text("Presiona ENTER para comenzar", 120, 400, 30, Color::GREEN);
            drop(d); // Termina el scope de dibujo antes de modificar rl
            if enter_pressed {
                show_instructions = false;
                rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
            }
            continue;
        }

        // Pantalla de éxito
        if show_success {
            let enter_pressed = rl.is_key_pressed(KeyboardKey::KEY_ENTER);
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::DARKBLUE);
            d.draw_text("¡FELICIDADES!", 200, 200, 50, Color::YELLOW);
            d.draw_text("¡Has llegado a la meta!", 180, 270, 30, Color::LIME);
            d.draw_text("Presiona ENTER para reiniciar", 150, 350, 30, Color::WHITE);
            drop(d);
            if enter_pressed {
                // Reiniciar jugador a la posición inicial
                player.x = (start_col as f32 + 0.5) * block_size as f32;
                player.y = (start_row as f32 + 0.5) * block_size as f32;
                player.angle = 0.0;
                show_success = false;
                show_instructions = true;
                rl.set_mouse_position((window_width as f32 / 2.0, window_height as f32 / 2.0));
            }
            continue;
        }

        // Obtener FPS antes de begin_drawing para evitar error de préstamos
        let fps = rl.get_fps();

        // Movimiento del jugador y colisiones con sonido
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

        // Guardar posición previa
        let prev_x = player.x;
        let prev_y = player.y;
        let next_x = player.x + dx;
        let next_y = player.y + dy;
        let mut collided = false;
        if !is_wall(next_x, player.y, block_size) {
            player.x = next_x;
        } else if tried_to_move && (dx != 0.0) {
            collided = true;
        }
        if !is_wall(player.x, next_y, block_size) {
            player.y = next_y;
        } else if tried_to_move && (dy != 0.0) {
            collided = true;
        }
        // Si hubo colisión, reproducir sonido
        if collided {
            println!("Intentando reproducir sonido de choque...");
            match File::open(bump_file_path) {
                Ok(file) => match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        fx_sink.append(source);
                        println!("Sonido de choque reproducido.");
                    }
                    Err(e) => {
                        println!("Error al decodificar el archivo de sonido: {}", e);
                    }
                },
                Err(e) => {
                    println!("Error al abrir el archivo de sonido: {}", e);
                }
            }
        }

        // Rotación con el mouse (horizontal)
        let mouse_x = rl.get_mouse_x();
        let center_x = window_width / 2;
        let delta_x = mouse_x - center_x;
        player.angle += delta_x as f32 * mouse_sensitivity;
        rl.set_mouse_position((center_x as f32, rl.get_mouse_y() as f32));

        // Detectar llegada a la meta
        let player_col = (player.x / block_size as f32) as usize;
        let player_row = (player.y / block_size as f32) as usize;
        if MAZE[player_row].chars().nth(player_col) == Some('E') {
            show_success = true;
            continue;
        }

        let time = rl.get_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKBLUE);
        // --- Renderizado 3D: cielo, piso y paredes bien diferenciadas ---
        for y in 0..window_height {
            for x in 0..window_width {
                if y < window_height / 2 {
                    d.draw_pixel(x as i32, y, Color::new(120, 180, 255, 255)); // cielo azul claro
                } else {
                    d.draw_pixel(x as i32, y, Color::new(60, 60, 60, 255)); // piso gris oscuro
                }
            }
        }
        for col in 0..window_width {
            let ray_angle = player.angle - fov / 2.0 + fov * (col as f32) / (window_width as f32);
            let mut dist = 0.0;
            let mut hit_wall = false;
            let mut wall_type = '#';
            let mut hit_x = 0.0;
            let mut hit_y = 0.0;
            while dist < 20.0 && !hit_wall {
                let rx = player.x + ray_angle.cos() * dist;
                let ry = player.y + ray_angle.sin() * dist;
                let mx = (rx / block_size as f32).floor() as isize;
                let my = (ry / block_size as f32).floor() as isize;
                if mx < 0 || my < 0 || mx >= MAZE[0].len() as isize || my >= MAZE.len() as isize {
                    hit_wall = true;
                    wall_type = '#';
                } else {
                    let cell = MAZE[my as usize].chars().nth(mx as usize).unwrap_or('#');
                    if cell != ' ' {
                        hit_wall = true;
                        wall_type = cell;
                        hit_x = rx / block_size as f32;
                        hit_y = ry / block_size as f32;
                    }
                }
                dist += 0.04;
            }
            // Corrección fish-eye
            let dist = dist * (player.angle - ray_angle).cos();
            // Proyección vertical
            let wall_height = (window_height as f32 * 1.2 / dist.max(0.2)).min(window_height as f32);
            let wall_top = (window_height as f32 / 2.0) - wall_height / 2.0;
            let wall_bottom = wall_top + wall_height;
            let mut color = match wall_type {
                '#' => Color::new(255, 0, 0, 255),      // rojo puro
                'A' => Color::new(0, 255, 0, 255),      // verde puro
                'B' => Color::new(0, 0, 255, 255),      // azul puro
                'C' => Color::new(255, 255, 0, 255),    // amarillo
                'E' => Color::new(255, 0, 255, 255),    // magenta
                _ => Color::DARKGRAY,
            };
            // Sombreado clásico: paredes verticales más oscuras
            let is_vertical = (hit_x.fract() < 0.05) || (hit_x.fract() > 0.95);
            if is_vertical {
                color = Color::new(
                    (color.r as f32 * 0.6) as u8,
                    (color.g as f32 * 0.6) as u8,
                    (color.b as f32 * 0.6) as u8,
                    255,
                );
            }
            // Degradado por distancia
            let fade = (1.0 - (dist / 30.0)).clamp(0.4, 1.0);
            color = Color::new(
                (color.r as f32 * fade) as u8,
                (color.g as f32 * fade) as u8,
                (color.b as f32 * fade) as u8,
                255,
            );
            d.draw_line_ex(
                Vector2 { x: col as f32, y: wall_top },
                Vector2 { x: col as f32, y: wall_bottom },
                2.0,
                color,
            );
        }
        // --- Minimapa 2D (esquina superior derecha) ---
        let minimap_scale = 0.25;
        let minimap_size = (window_width as f32 * minimap_scale) as i32;
        let minimap_x = window_width - minimap_size - 10;
        let minimap_y = 10;
        let mini_block = minimap_size / MAZE[0].len() as i32;
        for (row, line) in MAZE.iter().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                if cell != ' ' {
                    let x = minimap_x + (col as i32) * mini_block;
                    let y = minimap_y + (row as i32) * mini_block;
                    d.draw_rectangle(x, y, mini_block, mini_block, wall_color(cell));
                }
            }
        }
        // Punto inicial en azul
        let mini_start_x = minimap_x + (start_col as i32 * mini_block) + mini_block / 2;
        let mini_start_y = minimap_y + (start_row as i32 * mini_block) + mini_block / 2;
        d.draw_circle(mini_start_x, mini_start_y, (mini_block / 3) as f32, Color::BLUE);
        // Punto final en verde
        let mini_end_x = minimap_x + (end_col as i32 * mini_block) + mini_block / 2;
        let mini_end_y = minimap_y + (end_row as i32 * mini_block) + mini_block / 2;
        d.draw_circle(mini_end_x, mini_end_y, (mini_block / 3) as f32, Color::LIME);
        // Jugador en el minimapa
        let mini_px = minimap_x + (player.x / block_size as f32 * mini_block as f32) as i32;
        let mini_py = minimap_y + (player.y / block_size as f32 * mini_block as f32) as i32;
        // Animación de sprite: círculo del jugador palpita
        let pulse = (((time * 3.0).sin() * 0.5 + 1.0) * 0.5) as f32; // valor entre 0.0 y 1.0
        let animated_radius = (mini_block as f32 / 4.0) + pulse * (mini_block as f32 / 8.0);
        d.draw_circle(mini_px, mini_py, animated_radius, Color::YELLOW);
        let mini_dir_x = mini_px as f32 + player.angle.cos() * (mini_block as f32);
        let mini_dir_y = mini_py as f32 + player.angle.sin() * (mini_block as f32);
        d.draw_line(mini_px, mini_py, mini_dir_x as i32, mini_dir_y as i32, Color::WHITE);

        // Mostrar FPS en la esquina inferior derecha
        let fps_color = if fps > 15 { Color::GREEN } else { Color::RED };
        let fps_text = format!("FPS: {}", fps);
        let text_size = d.measure_text(&fps_text, 20);
        d.draw_text(&fps_text, window_width - text_size - 20, window_height - 40, 20, fps_color);

        // Instrucciones claras en pantalla
        d.draw_text("Controles:", 10, 10, 20, Color::WHITE);
        d.draw_text("W/S: Avanzar / Retroceder", 10, 30, 18, Color::LIGHTGRAY);
        d.draw_text("A/D: Girar a la izquierda / derecha", 10, 50, 18, Color::LIGHTGRAY);
        d.draw_text("Mouse: Girar la cámara horizontalmente", 10, 70, 18, Color::LIGHTGRAY);
        d.draw_text("ESC: Salir", 10, 90, 18, Color::LIGHTGRAY);
    }
    // Detener música al salir
    sink.stop();
}
