use std::time::{Duration, Instant};

use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
};

mod constants;
mod map;
mod player;
mod renderer;

use constants::{MOVE_SPEED, ROT_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH, TARGET_FPS};
use map::{Map, WORLD_MAP};
use player::Player;
use renderer::draw_frame;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raycast", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_time = Duration::from_nanos(1_000_000_000u64 / TARGET_FPS as u64);
    let world_map: &Map = &WORLD_MAP;
    let mut player = Player::new();

    'running: loop {
        let frame_start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let dt = frame_start.elapsed().as_secs_f32();
        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::W) {
            player.move_forward(world_map, MOVE_SPEED * dt);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            player.move_backward(world_map, MOVE_SPEED * dt);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            player.strafe_left(world_map, MOVE_SPEED * dt);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            player.strafe_right(world_map, MOVE_SPEED * dt);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            player.rotate_left(ROT_SPEED * dt);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            player.rotate_right(ROT_SPEED * dt);
        }

        draw_frame(&mut canvas, world_map, &player);

        canvas.present();

        let elapsed = frame_start.elapsed();
        if elapsed < frame_time {
            ::std::thread::sleep(frame_time - elapsed);
        }
    }
}
