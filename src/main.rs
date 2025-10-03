use std::{time::Duration, usize};

use sdl2::{
    self,
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
};

const MAP_WIDTH: i32 = 24;
const MAP_HEIGHT: i32 = 24;
const SCREEN_WIDTH: u32 = 600;
const SCREEN_HEIGTH: u32 = 400;
const TILE_SIZE: i32 = 32;

fn world_map() -> Vec<Vec<i32>> {
    vec![
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ],
        vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ],
    ]
}

struct Player {
    pos_x: f32,
    pos_y: f32,
    dir_x: f32,
    dir_y: f32,
    plane_x: f32,
    plane_y: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            pos_x: 22.0,
            pos_y: 12.0,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Raycast", SCREEN_WIDTH, SCREEN_HEIGTH)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let move_speed = 0.1;
    let rot_speed = 0.05;
    let mut world_map = world_map();
    let mut player = Player::new();
    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();

    for x in 0..SCREEN_WIDTH {
        // Calculate ray position
        let camera_x = 2.0 * x as f32 / SCREEN_WIDTH as f32 - 1.0;
        let ray_dir_x = player.dir_x + player.plane_x * camera_x;
        let ray_dir_y = player.dir_y + player.plane_y * camera_x;

        // Which box of the map we're in
        let mut map_x = player.pos_x as i32;
        let mut map_y = player.pos_y as i32;

        // Lenght of ray from current position to next x or y-side
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
            (map_x as f32 + 1.0 - player.pos_x)
        };
        let mut side_dist_y = if ray_dir_y < 0.0 {
            (player.pos_y - map_y as f32) * delta_dist_y
        } else {
            (map_y as f32 + 1.0 - player.pos_y) * delta_dist_y
        };

        // What direction to step in x or y direction (either +1 or -1)
        let step_x = if ray_dir_x < 0.0 { -1 } else { 1 };
        let step_y = if ray_dir_y < 0.0 { -1 } else { 1 };

        let mut hit = 0;
        let mut side = 0;

        // Perform DDA
        while hit == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            if map_x >= 0 && map_x < MAP_WIDTH as i32 && map_y >= 0 && map_y < MAP_HEIGHT as i32 {
                if world_map[map_x as usize][map_y as usize] > 0 {
                    hit = 1;
                }
            } else {
                break;
            }
        }
        // Calculate distance to the wall
        let perp_wall_dist = if side == 0 {
            (map_x as f32 - player.pos_x + (1 - step_x) as f32 / 2.0) / ray_dir_x
        } else {
            (map_y as f32 - player.pos_y + (1 - step_y) as f32 / 2.0) / ray_dir_y
        };

        // Calculate heigth of line to draw on screen
        let line_height = (SCREEN_HEIGTH as f32 / perp_wall_dist) as i32;

        // Calculate lowest and highest pixel to fill in current stripe
        let draw_startr = (-line_height / 2 + SCREEN_HEIGTH as i32 / 2).max(0) as i32;
        let draw_end =
            (line_height / 2 + SCREEN_HEIGTH as i32 / 2).min(SCREEN_HEIGTH as i32) as i32;

        // Draw the wall slice
        let color = if side == 1 {
            Color::RGB(128, 128, 128)
        } else {
            Color::RGB(255, 255, 255)
        };
        canvas.set_draw_color(color);
        canvas
            .draw_line((x as i32, draw_startr), (x as i32, draw_end))
            .unwrap();
    }

    canvas.set_draw_color(Color::RGB(50, 50, 50));
    canvas
        .fill_rect(Rect::new(
            0,
            SCREEN_HEIGTH as i32 / 2,
            SCREEN_WIDTH as u32,
            (SCREEN_HEIGTH / 2) as u32,
        ))
        .unwrap();

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
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
