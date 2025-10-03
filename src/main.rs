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

    fn move_forward(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x + self.dir_x * move_speed;
        let new_y = self.pos_y + self.dir_y * move_speed;

        // Check collision
        if world_map[new_x as usize][new_y as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    fn move_backward(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x - self.dir_x * move_speed;
        let new_y = self.pos_y - self.dir_y * move_speed;

        if world_map[new_x as usize][new_y as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    fn strafe_left(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x - self.plane_x * move_speed;
        let new_y = self.pos_y - self.plane_y * move_speed;

        if world_map[new_x as usize][new_y as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    fn strafe_right(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x + self.plane_x * move_speed;
        let new_y = self.pos_y + self.plane_y * move_speed;

        if world_map[new_x as usize][new_y as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    fn rotate_left(&mut self, rot_speed: f32) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * rot_speed.cos() - self.dir_y * rot_speed.sin();
        self.dir_y = old_dir_x * rot_speed.sin() + self.dir_y * rot_speed.cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * rot_speed.cos() - self.plane_y * rot_speed.sin();
        self.plane_y = old_plane_x * rot_speed.sin() + self.plane_y * rot_speed.cos();
    }

    fn rotate_right(&mut self, rot_speed: f32) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * (-rot_speed).cos() - self.dir_y * (-rot_speed).sin();
        self.dir_y = old_dir_x * (-rot_speed).sin() + self.dir_y * (-rot_speed).cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * (-rot_speed).cos() - self.plane_y * (-rot_speed).sin();
        self.plane_y = old_plane_x * (-rot_speed).sin() + self.plane_y * (-rot_speed).cos();
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

    // TODO: Add player movement.
    let move_speed = 0.1;
    let rot_speed = 0.05;
    let mut world_map = world_map();
    let mut player = Player::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            // TODO: Add player movement.
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Handle continupus keyboard input
        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::W) {
            player.move_forward(&world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            player.move_backward(&world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            player.strafe_left(&world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            player.strafe_right(&world_map, move_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            player.rotate_left(rot_speed);
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            player.rotate_right(rot_speed);
        }

        // Clear screen with ceiling color
        canvas.set_draw_color(Color::RGB(64, 64, 64));
        canvas.clear();

        // Draw floor
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas
            .fill_rect(Rect::new(
                0,
                SCREEN_HEIGTH as i32 / 2,
                SCREEN_WIDTH as u32,
                (SCREEN_HEIGTH / 2) as u32,
            ))
            .unwrap();

        // Raycasting
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
                (map_x as f32 + 1.0 - player.pos_x) * delta_dist_x
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
                if map_x >= 0 && map_x < MAP_WIDTH as i32 && map_y >= 0 && map_y < MAP_HEIGHT as i32
                {
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

            // Choose wall color based on wall type and side
            let wall_type = world_map[map_y as usize][map_x as usize];
            let base_color = match wall_type {
                1 => (255, 0, 0),     // Red
                2 => (0, 255, 0),     // Green
                3 => (0, 0, 255),     // Blue
                4 => (255, 255, 0),   // Yellow
                5 => (255, 0, 255),   // Magenta
                _ => (255, 255, 255), // White
            };
            // Draw the wall slice
            let color = if side == 1 {
                Color::RGB(base_color.0 / 2, base_color.1 / 2, base_color.2 / 2)
            } else {
                Color::RGB(base_color.0, base_color.1, base_color.2)
            };
            canvas.set_draw_color(color);
            canvas
                .draw_line((x as i32, draw_startr), (x as i32, draw_end))
                .unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
