use crate::map::{Map, is_wall};

pub const RADIUS: f32 = 0.25;

pub struct Player {
    pub pos_x: f32,
    pub pos_y: f32,
    pub dir_x: f32,
    pub dir_y: f32,
    pub plane_x: f32,
    pub plane_y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos_x: 22.0,
            pos_y: 12.0,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }

    pub fn move_forward(&mut self, world_map: &Map, move_speed: f32) {
        self.try_move(world_map, self.dir_x * move_speed, self.dir_y * move_speed);
    }

    pub fn move_backward(&mut self, world_map: &Map, move_speed: f32) {
        self.try_move(
            world_map,
            -self.dir_x * move_speed,
            -self.dir_y * move_speed,
        );
    }

    pub fn strafe_left(&mut self, world_map: &Map, move_speed: f32) {
        self.try_move(
            world_map,
            -self.plane_x * move_speed,
            -self.plane_y * move_speed,
        );
    }

    pub fn strafe_right(&mut self, world_map: &Map, move_speed: f32) {
        self.try_move(
            world_map,
            self.plane_x * move_speed,
            self.plane_y * move_speed,
        );
    }

    pub fn rotate_left(&mut self, rot_speed: f32) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * rot_speed.cos() - self.dir_y * rot_speed.sin();
        self.dir_y = old_dir_x * rot_speed.sin() + self.dir_y * rot_speed.cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * rot_speed.cos() - self.plane_y * rot_speed.sin();
        self.plane_y = old_plane_x * rot_speed.sin() + self.plane_y * rot_speed.cos();
    }

    pub fn rotate_right(&mut self, rot_speed: f32) {
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * (-rot_speed).cos() - self.dir_y * (-rot_speed).sin();
        self.dir_y = old_dir_x * (-rot_speed).sin() + self.dir_y * (-rot_speed).cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * (-rot_speed).cos() - self.plane_y * (-rot_speed).sin();
        self.plane_y = old_plane_x * (-rot_speed).sin() + self.plane_y * (-rot_speed).cos();
    }

    fn try_move(&mut self, world_map: &Map, dx: f32, dy: f32) {
        let new_x = self.pos_x + dx;
        if !Self::collides(world_map, new_x, self.pos_y) {
            self.pos_x = new_x;
        }

        let new_y = self.pos_y + dy;
        if !Self::collides(world_map, self.pos_x, new_y) {
            self.pos_y = new_y;
        }
    }

    fn collides(world_map: &Map, x: f32, y: f32) -> bool {
        let min_x = (x - RADIUS).floor() as i32;
        let max_x = (x + RADIUS).floor() as i32;
        let min_y = (y - RADIUS).floor() as i32;
        let max_y = (y + RADIUS).floor() as i32;

        (min_y..=max_y).any(|cy| (min_x..=max_x).any(|cx| is_wall(world_map, cx, cy)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
    use crate::map::WORLD_MAP;

    fn player_at(pos_x: f32, pos_y: f32, dir_x: f32, dir_y: f32) -> Player {
        Player {
            pos_x,
            pos_y,
            dir_x,
            dir_y,
            plane_x: 0.0,
            plane_y: 0.66,
        }
    }

    #[test]
    fn spawn_defaults() {
        let player = Player::new();
        assert_eq!(player.pos_x, 22.0);
        assert_eq!(player.pos_y, 12.0);
        assert_eq!(player.dir_x, -1.0);
        assert_eq!(player.dir_y, 0.0);
    }

    #[test]
    fn move_forward_displaces_along_direction() {
        let mut player = player_at(10.5, 10.5, 1.0, 0.0);
        player.move_forward(&WORLD_MAP, 0.5);
        assert!((player.pos_x - 11.0).abs() < 1e-6);
        assert!((player.pos_y - 10.5).abs() < 1e-6);
    }

    #[test]
    fn move_backward_displaces_against_direction() {
        let mut player = player_at(10.5, 10.5, 1.0, 0.0);
        player.move_backward(&WORLD_MAP, 0.5);
        assert!((player.pos_x - 10.0).abs() < 1e-6);
    }

    #[test]
    fn strafe_right_displaces_along_camera_plane() {
        let mut player = player_at(10.5, 10.5, 1.0, 0.0);
        player.plane_x = 0.0;
        player.plane_y = 0.66;
        player.strafe_right(&WORLD_MAP, 1.0);
        assert!((player.pos_x - 10.5).abs() < 1e-6);
        assert!((player.pos_y - 11.16).abs() < 1e-4);
    }

    #[test]
    fn movement_into_wall_is_blocked() {
        let mut player = player_at(22.7, 12.5, 1.0, 0.0);
        player.move_forward(&WORLD_MAP, 0.5);
        assert!((player.pos_x - 22.7).abs() < 1e-6);
        assert!((player.pos_y - 12.5).abs() < 1e-6);
    }

    #[test]
    fn sliding_moves_along_wall_when_blocked_head_on() {
        let mut player = player_at(22.7, 12.5, 1.0, 0.0);
        player.strafe_right(&WORLD_MAP, 0.3);
        assert!((player.pos_x - 22.7).abs() < 1e-6);
        assert!(player.pos_y > 12.5);
    }

    #[test]
    fn radius_blocks_movement_before_center_enters_wall_cell() {
        let mut map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        map[12][11] = 1;
        let mut player = player_at(9.5, 12.5, 1.0, 0.0);

        player.move_forward(&map, 1.0);
        assert!((player.pos_x - 10.5).abs() < 1e-6);

        player.move_forward(&map, 1.0);
        assert!(
            (player.pos_x - 10.5).abs() < 1e-6,
            "second step must be blocked"
        );
    }

    #[test]
    fn diagonal_move_slides_around_corner() {
        let mut map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        map[12][12] = 1;
        let mut player = player_at(11.7, 11.5, 1.0, 1.0);

        player.move_forward(&map, 0.25);

        assert!((player.pos_x - 11.95).abs() < 1e-6, "x axis advances");
        assert!(
            (player.pos_y - 11.5).abs() < 1e-6,
            "y axis blocked by corner"
        );
    }

    #[test]
    fn out_of_bounds_is_treated_as_solid() {
        let empty_map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        let mut player = player_at(0.5, 0.5, -1.0, -1.0);
        player.move_forward(&empty_map, 1.0);
        assert!((player.pos_x - 0.5).abs() < 1e-6);
        assert!((player.pos_y - 0.5).abs() < 1e-6);
    }

    #[test]
    fn rotation_preserves_direction_length_and_perpendicular_plane() {
        let mut player = player_at(5.0, 5.0, 1.0, 0.0);
        player.rotate_left(0.7);

        let dir_len = (player.dir_x * player.dir_x + player.dir_y * player.dir_y).sqrt();
        assert!((dir_len - 1.0).abs() < 1e-5);

        let dot = player.dir_x * player.plane_x + player.dir_y * player.plane_y;
        assert!(dot.abs() < 1e-5, "plane must stay perpendicular to dir");
    }

    #[test]
    fn rotate_left_then_right_restores_orientation() {
        let mut player = player_at(5.0, 5.0, 1.0, 0.0);
        player.rotate_left(0.37);
        player.rotate_right(0.37);

        assert!((player.dir_x - 1.0).abs() < 1e-5);
        assert!(player.dir_y.abs() < 1e-5);
        assert!(player.plane_x.abs() < 1e-5);
        assert!((player.plane_y - 0.66).abs() < 1e-5);
    }
}
