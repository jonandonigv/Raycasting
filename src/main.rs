use std::time::{Duration, Instant};

use sdl2::{
    VideoSubsystem,
    event::Event,
    keyboard::{Keycode, Scancode},
    render::Canvas,
    video::Window,
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

    let (mut canvas, vsync) = build_canvas(&video_subsystem);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_time = Duration::from_nanos(1_000_000_000u64 / TARGET_FPS as u64);
    let world_map: &Map = &WORLD_MAP;
    let mut player = Player::new();
    let mut fps_counter = FpsCounter::new(Instant::now());

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

        if !vsync {
            pace_frame(frame_start, frame_time);
        }

        if let Some(fps) = fps_counter.tick(frame_start) {
            let mode = if vsync { " (vsync)" } else { "" };
            let _ = canvas
                .window_mut()
                .set_title(&format!("Raycast - {} FPS{}", fps, mode));
        }
    }
}

fn build_canvas(video_subsystem: &VideoSubsystem) -> (Canvas<Window>, bool) {
    let window = video_subsystem
        .window("Raycast", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    if let Ok(canvas) = window.into_canvas().accelerated().present_vsync().build() {
        return (canvas, true);
    }

    let window = video_subsystem
        .window("Raycast", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    (canvas, false)
}

const SPIN_THRESHOLD: Duration = Duration::from_micros(1500);

fn pace_frame(frame_start: Instant, frame_time: Duration) {
    let Some(remaining) = frame_time.checked_sub(frame_start.elapsed()) else {
        return;
    };
    if remaining > SPIN_THRESHOLD {
        ::std::thread::sleep(remaining - SPIN_THRESHOLD);
    }
    while frame_start.elapsed() < frame_time {
        ::std::hint::spin_loop();
    }
}

struct FpsCounter {
    frames: u32,
    window_start: Instant,
}

impl FpsCounter {
    fn new(now: Instant) -> Self {
        Self {
            frames: 0,
            window_start: now,
        }
    }

    fn tick(&mut self, now: Instant) -> Option<u32> {
        self.frames += 1;
        let elapsed = now - self.window_start;
        if elapsed >= Duration::from_secs(1) {
            let fps = (self.frames as f64 / elapsed.as_secs_f64()).round() as u32;
            self.frames = 0;
            self.window_start = now;
            Some(fps)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pace_frame_never_returns_before_target() {
        let start = Instant::now();
        pace_frame(start, Duration::from_millis(8));
        assert!(start.elapsed() >= Duration::from_millis(8));
    }

    #[test]
    fn pace_frame_returns_immediately_when_frame_overran() {
        let start = Instant::now();
        ::std::thread::sleep(Duration::from_millis(5));
        pace_frame(start, Duration::from_millis(2));
        assert!(start.elapsed() < Duration::from_millis(100));
    }

    #[test]
    fn fps_counter_reports_once_per_second_and_resets() {
        let t0 = Instant::now();
        let mut counter = FpsCounter::new(t0);

        assert_eq!(counter.tick(t0), None);
        assert_eq!(counter.tick(t0 + Duration::from_millis(500)), None);
        assert_eq!(counter.tick(t0 + Duration::from_secs(1)), Some(3));

        assert_eq!(counter.tick(t0 + Duration::from_millis(1500)), None);
        assert_eq!(counter.tick(t0 + Duration::from_secs(2)), Some(2));
    }

    #[test]
    fn fps_counter_rounds_to_nearest_whole_frame_rate() {
        let t0 = Instant::now();
        let mut counter = FpsCounter::new(t0);

        assert_eq!(counter.tick(t0), None);
        assert_eq!(counter.tick(t0 + Duration::from_millis(300)), None);
        assert_eq!(counter.tick(t0 + Duration::from_millis(600)), None);
        assert_eq!(counter.tick(t0 + Duration::from_millis(900)), None);
        assert_eq!(counter.tick(t0 + Duration::from_millis(1200)), Some(4));
    }
}
