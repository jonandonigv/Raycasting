pub const MAP_WIDTH: usize = 24;
pub const MAP_HEIGHT: usize = 24;
pub const SCREEN_WIDTH: u32 = 600;
pub const SCREEN_HEIGHT: u32 = 400;
pub const MOVE_SPEED: f32 = 9.0;
pub const ROT_SPEED: f32 = 3.0;
pub const TARGET_FPS: u32 = 60;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speeds_match_expected_per_frame_displacement() {
        let frame_dt = 1.0 / TARGET_FPS as f32;
        assert!((MOVE_SPEED * frame_dt - 0.15).abs() < 1e-4);
        assert!((ROT_SPEED * frame_dt - 0.05).abs() < 1e-4);
    }
}
