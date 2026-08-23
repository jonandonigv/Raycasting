# Raycasting Engine

A simple real-time raycasting renderer written in Rust, implementing classic Wolfenstein 3D-style 3D rendering using SDL2.

## Overview

This project demonstrates the fundamentals of raycasting - a rendering technique that creates a 3D perspective from a 2D map by casting rays and calculating wall intersections. It features real-time player movement, collision detection, and shaded walls for depth perception.

## Features

- **Real-time 3D Rendering**: Classic raycasting implementation using the DDA (Digital Differential Analyzer) algorithm
- **Player Movement**: WASD movement with arrow key rotation and strafing support, scaled by real elapsed time (delta-time)
- **Collision Detection**: Player has a collision radius and slides along walls; cannot clip through corners
- **Depth Shading**: Walls are shaded darker when hit on the side for realistic depth
- **Multiple Wall Types**: 5 different colored wall types
- **60 FPS Game Loop**: Delta-timed loop that sleeps only the remaining frame budget

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
- **Frame Rate**: Target 60 FPS via delta-time pacing
- **Rendering**: Software-based (no GPU acceleration required)

### Architecture

- `main.rs`: SDL2 setup, input handling, and the delta-timed game loop
- `renderer.rs`: Pure DDA raycasting (`cast_ray`), wall color mapping (`wall_color`), and frame drawing (`draw_frame`)
- `player.rs`: Position, direction vector, and camera plane; radius-based collision with independent X/Y resolution (wall sliding) and rotation math
- `map.rs`: Static 24x24 world grid as a `const` array with bounds-safe accessors (`is_wall`, `wall_type`) that treat out-of-bounds as solid
- `constants.rs`: Screen/map dimensions, movement/rotation speeds, and target FPS
- Unit tests live alongside each module under `#[cfg(test)]`; run with `cargo test`

## Development Workflow

This project follows a gitflow-style branching model:

- `master`: production-ready code only
- `develop`: integration branch; all work merges here first
- `feature/<name>`: branches cut from `develop` for each change, landed through GitHub PRs with merge commits
- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `refactor:`, `test:`, `docs:`)

Every PR must pass the verification gate before merging:

```bash
cargo build && cargo test && cargo clippy --all-targets -- -D warnings && cargo fmt --check
```

## Dependencies

- `sdl2` (v0.38.0) - SDL2 bindings for windowing, input, and rendering

## License

[Add your license here]

## Acknowledgments

This implementation is based on the classic raycasting technique popularized by games like Wolfenstein 3D.
