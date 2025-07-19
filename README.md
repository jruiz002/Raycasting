# Raycasting Laberinto (Rust + Raylib)

## Descripción
Este proyecto es un juego de laberinto en primera persona implementado en Rust usando Raylib y rodio. El motor utiliza raycasting para simular un entorno 3D retro tipo Wolfenstein 3D. El objetivo es recorrer el laberinto desde el punto inicial hasta la meta, sin atravesar paredes.

## Características
- Motor de raycasting 3D simple y jugable
- Movimiento y rotación de cámara (teclado y mouse)
- Minimapa animado en la esquina superior derecha
- Pantalla de bienvenida e instrucciones
- Pantalla de éxito al llegar a la meta
- Música de fondo y efectos de sonido al chocar
- Animación de sprite en el minimapa
- FPS mostrados en pantalla

## Controles
- **W/S**: Avanzar / Retroceder
- **A/D**: Girar a la izquierda / derecha
- **Mouse**: Girar la cámara horizontalmente
- **ESC**: Salir del juego
- **ENTER**: Comenzar desde la pantalla de bienvenida o reiniciar tras ganar

## Requisitos
- Rust (https://www.rust-lang.org/)
- Raylib instalado en el sistema (por ejemplo, con Homebrew: `brew install raylib`)
- Archivos de audio en la carpeta `assets/`:
  - `laberinto.mp3` (música de fondo)
  - `bump.wav` (efecto de choque)

## Instalación y ejecución
1. **Clona el repositorio o descarga el código fuente.**
2. **Instala las dependencias:**
   - Raylib (nativo):
     ```sh
     brew install raylib
     ```
   - Rust y cargo (si no los tienes):
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
3. **Coloca los archivos de audio en la carpeta `assets/`**
   - `assets/laberinto.mp3`
   - `assets/bump.wav`
4. **Compila el proyecto:**
   ```sh
   cargo build
   ```
5. **Ejecuta el juego:**
   ```sh
   cargo run
   ```

## Notas
- Puedes modificar el laberinto editando la constante `MAZE` en `src/main.rs`.
- Para agregar más niveles, puedes crear más mapas y modificar la lógica de selección.
- Si tienes problemas con el audio, asegúrate de que los archivos existen y están en el formato correcto.

---
¡Disfruta explorando el laberinto y buena suerte! 