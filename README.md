# Raycasting 3D (Raylib)

Este proyecto implementa un raycaster 3D sencillo usando Raylib, organizado en módulos y renderizado a un framebuffer.

## Estructura
- `src/main.rs`: bucle principal del juego y orquestación.
- `src/framebuffer.rs`: manejo de framebuffer
- `src/renderer.rs`: render de cielo/suelo y muros por raycasting, dibujando en el framebuffer.
- `src/ui.rs`: HUD y minimapa dibujados sobre la escena.
- `src/player.rs`: estado y utilidades del jugador.
- `src/maze.rs`: definición del laberinto y utilidades de acceso.

## Framebuffer
- Se crea un framebuffer con `Framebuffer::new`.
- Se dibuja la escena 3D con `Framebuffer::begin(...)` y `renderer::render_scene(...)`.
- Se presenta el resultado en pantalla con `Framebuffer::draw_to_screen(...)` usando `draw_texture_rec` con flip vertical.

## Controles
- `W/S`: avanzar/retroceder
- `A/D`: girar
- Mouse: mirar
- `ESC`: salir

## Construcción y ejecución
- Requisitos: Rust estable, Raylib para Rust (vía crate `raylib`), cargo.
- Compilar: `cargo build -q`
- Ejecutar: `cargo run -q`

## Organización del código
- La escena 3D (cielo/suelo/muros) se dibuja dentro del framebuffer.
- El HUD y minimapa se dibujan sobre el resultado del framebuffer.
- El efecto de linterna oscurece los bordes sin tapar el centro.

