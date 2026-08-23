use std::time::Duration;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
};

mod constants;
mod map;
mod player;

use constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use map::{Map, WORLD_MAP, is_wall, wall_type};
use player::Player;

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

    let move_speed = 0.1;
    let rot_speed = 0.05;
    let world_map: &Map = &WORLD_MAP;
    let mut player = Player::new();

    'running: loop {
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

        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::W) {
            player.move_forward(world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            player.move_backward(world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            player.strafe_left(world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            player.strafe_right(world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            player.rotate_left(rot_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            player.rotate_right(rot_speed);
        }

        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas
            .fill_rect(Rect::new(
                0,
                SCREEN_HEIGHT as i32 / 2,
                SCREEN_WIDTH,
                SCREEN_HEIGHT / 2,
            ))
            .unwrap();

        for x in 0..SCREEN_WIDTH {
            let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
            let ray_dir_x = player.dir_x + player.plane_x * camera_x;
            let ray_dir_y = player.dir_y + player.plane_y * camera_x;

            let mut map_x = player.pos_x as i32;
            let mut map_y = player.pos_y as i32;

            let delta_dist_x = if ray_dir_x == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_x).abs()
            };
            let delta_dist_y = if ray_dir_y == 0.0 {
                1e30
            } else {
                (1.0 / ray_dir_y).abs()
            };

            let mut side_dist_x = if ray_dir_x < 0.0 {
                (player.pos_x - map_x as f32) * delta_dist_x
            } else {
                (map_x as f32 + 1.0 - player.pos_x) * delta_dist_x
            };
            let mut side_dist_y = if ray_dir_y < 0.0 {
                (player.pos_y - map_y as f32) * delta_dist_y
            } else {
                (map_y as f32 + 1.0 - player.pos_y) * delta_dist_y
            };

            let step_x = if ray_dir_x < 0.0 { -1 } else { 1 };
            let step_y = if ray_dir_y < 0.0 { -1 } else { 1 };

            let mut side = 0;

            while !is_wall(world_map, map_x, map_y) {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }
            }
            let perp_wall_dist = if side == 0 {
                (map_x as f32 - player.pos_x + (1 - step_x) as f32 / 2.0) / ray_dir_x
            } else {
                (map_y as f32 - player.pos_y + (1 - step_y) as f32 / 2.0) / ray_dir_y
            };

            let line_height = (SCREEN_HEIGHT as f32 / perp_wall_dist) as i32;

            let draw_start = (-line_height / 2 + SCREEN_HEIGHT as i32 / 2).max(0);
            let draw_end = (line_height / 2 + SCREEN_HEIGHT as i32 / 2).min(SCREEN_HEIGHT as i32);

            let Some(wall_type) = wall_type(world_map, map_x, map_y) else {
                continue;
            };
            let base_color = match wall_type {
                1 => (255, 0, 0),
                2 => (0, 255, 0),
                3 => (0, 0, 255),
                4 => (255, 255, 0),
                5 => (255, 0, 255),
                _ => (255, 255, 255),
            };
            let color = if side == 1 {
                Color::RGB(base_color.0 / 2, base_color.1 / 2, base_color.2 / 2)
            } else {
                Color::RGB(base_color.0, base_color.1, base_color.2)
            };
            canvas.set_draw_color(color);
            canvas
                .draw_line((x as i32, draw_start), (x as i32, draw_end))
                .unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
