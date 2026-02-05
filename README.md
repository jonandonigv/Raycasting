# Raycasting Engine

A simple real-time raycasting renderer written in Rust, implementing classic Wolfenstein 3D-style 3D rendering using SDL2.

## Overview

This project demonstrates the fundamentals of raycasting - a rendering technique that creates a 3D perspective from a 2D map by casting rays and calculating wall intersections. It features real-time player movement, collision detection, and shaded walls for depth perception.

## Features

- **Real-time 3D Rendering**: Classic raycasting implementation using the DDA (Digital Differential Analyzer) algorithm
- **Player Movement**: WASD movement with arrow key rotation and strafing support
- **Collision Detection**: Player cannot walk through walls
- **Depth Shading**: Walls are shaded darker when hit on the side for realistic depth
- **Multiple Wall Types**: 5 different colored wall types
- **60 FPS Game Loop**: Smooth rendering with frame rate limiting

## Controls

| Key | Action |
|-----|--------|
| `W` | Move forward |
| `S` | Move backward |
| `A` | Strafe left |
| `D` | Strafe right |
| `←` (Left Arrow) | Rotate left |
| `→` (Right Arrow) | Rotate right |
| `ESC` | Quit game |

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [SDL2](https://www.libsdl.org/) development libraries

### Installing SDL2

**macOS:**
```bash
brew install sdl2
```

**Ubuntu/Debian:**
```bash
sudo apt-get install libsdl2-dev
```

**Windows:**
Download SDL2 development libraries from the official website and follow the setup instructions.

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd Raycasting

# Build and run
cargo run

# Or build in release mode for better performance
cargo run --release
```

## Technical Details

- **Resolution**: 600x400 pixels
- **Map Size**: 24x24 grid
- **Frame Rate**: 60 FPS
- **Rendering**: Software-based (no GPU acceleration required)

### Architecture

- `Player` struct: Manages position, direction, camera plane, and movement
- `world_map()`: Hardcoded 24x24 game world with 5 wall types
- Raycasting loop: Casts rays for each screen column, detects wall hits using DDA, calculates distances and draws vertical wall slices

## Dependencies

- `sdl2` (v0.38.0) - SDL2 bindings for windowing, input, and rendering

## License

[Add your license here]

## Acknowledgments

This implementation is based on the classic raycasting technique popularized by games like Wolfenstein 3D.
