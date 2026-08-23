use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::map::{Map, is_wall, wall_type};
use crate::player::Player;

pub struct RayHit {
    pub side: u8,
    pub perp_dist: f32,
    pub wall_type: i32,
}

pub fn cast_ray(
    map: &Map,
    pos_x: f32,
    pos_y: f32,
    ray_dir_x: f32,
    ray_dir_y: f32,
) -> Option<RayHit> {
    let mut map_x = pos_x.floor() as i32;
    let mut map_y = pos_y.floor() as i32;

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
        (pos_x - map_x as f32) * delta_dist_x
    } else {
        (map_x as f32 + 1.0 - pos_x) * delta_dist_x
    };
    let mut side_dist_y = if ray_dir_y < 0.0 {
        (pos_y - map_y as f32) * delta_dist_y
    } else {
        (map_y as f32 + 1.0 - pos_y) * delta_dist_y
    };

    let step_x = if ray_dir_x < 0.0 { -1 } else { 1 };
    let step_y = if ray_dir_y < 0.0 { -1 } else { 1 };

    let mut side = 0;

    while !is_wall(map, map_x, map_y) {
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

    let perp_dist = if side == 0 {
        (map_x as f32 - pos_x + (1 - step_x) as f32 / 2.0) / ray_dir_x
    } else {
        (map_y as f32 - pos_y + (1 - step_y) as f32 / 2.0) / ray_dir_y
    };

    let hit_type = wall_type(map, map_x, map_y)?;

    Some(RayHit {
        side: side as u8,
        perp_dist,
        wall_type: hit_type,
    })
}

pub fn wall_color(wall_type: i32, side: u8) -> (u8, u8, u8) {
    let base = match wall_type {
        1 => (255, 0, 0),
        2 => (0, 255, 0),
        3 => (0, 0, 255),
        4 => (255, 255, 0),
        5 => (255, 0, 255),
        _ => (255, 255, 255),
    };
    if side == 1 {
        (base.0 / 2, base.1 / 2, base.2 / 2)
    } else {
        base
    }
}

pub fn draw_frame(canvas: &mut Canvas<Window>, map: &Map, player: &Player) {
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

        let Some(hit) = cast_ray(map, player.pos_x, player.pos_y, ray_dir_x, ray_dir_y) else {
            continue;
        };

        let line_height = (SCREEN_HEIGHT as f32 / hit.perp_dist) as i32;

        let draw_start = (-line_height / 2 + SCREEN_HEIGHT as i32 / 2).max(0);
        let draw_end = (line_height / 2 + SCREEN_HEIGHT as i32 / 2).min(SCREEN_HEIGHT as i32);

        let (r, g, b) = wall_color(hit.wall_type, hit.side);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas
            .draw_line((x as i32, draw_start), (x as i32, draw_end))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
    use crate::map::WORLD_MAP;

    fn open_south_only_map() -> Map {
        let mut map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        map[MAP_HEIGHT - 1] = [1; MAP_WIDTH];
        map
    }

    #[test]
    fn ray_west_from_spawn_hits_border_at_full_distance() {
        let hit = cast_ray(&WORLD_MAP, 22.0, 12.0, -1.0, 0.0).unwrap();
        assert!((hit.perp_dist - 21.0).abs() < 1e-4);
        assert_eq!(hit.side, 0);
        assert_eq!(hit.wall_type, 1);
    }

    #[test]
    fn ray_east_from_spawn_hits_near_wall() {
        let hit = cast_ray(&WORLD_MAP, 22.0, 12.0, 1.0, 0.0).unwrap();
        assert!((hit.perp_dist - 1.0).abs() < 1e-4);
        assert_eq!(hit.side, 0);
        assert_eq!(hit.wall_type, 1);
    }

    #[test]
    fn ray_south_from_spawn_hits_side_face() {
        let hit = cast_ray(&WORLD_MAP, 22.0, 12.0, 0.0, 1.0).unwrap();
        assert!((hit.perp_dist - 11.0).abs() < 1e-4);
        assert_eq!(hit.side, 1);
        assert_eq!(hit.wall_type, 1);
    }

    #[test]
    fn ray_hits_interior_block_with_expected_type_and_side() {
        let mut map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        map[12][12] = 3;
        let hit = cast_ray(&map, 9.5, 12.5, 1.0, 0.0).unwrap();
        assert!((hit.perp_dist - 2.5).abs() < 1e-4);
        assert_eq!(hit.side, 0);
        assert_eq!(hit.wall_type, 3);
    }

    #[test]
    fn ray_through_open_border_returns_none() {
        let map = open_south_only_map();
        let result = cast_ray(&map, 12.0, 12.0, 0.0, -1.0);
        assert!(result.is_none());
    }

    #[test]
    fn custom_wall_type_is_reported() {
        let mut map = [[0i32; MAP_WIDTH]; MAP_HEIGHT];
        map[MAP_HEIGHT - 1] = [5; MAP_WIDTH];
        let hit = cast_ray(&map, 12.0, 12.0, 0.0, 1.0).unwrap();
        assert_eq!(hit.wall_type, 5);
    }

    #[test]
    fn wall_color_matches_wall_types() {
        assert_eq!(wall_color(1, 0), (255, 0, 0));
        assert_eq!(wall_color(2, 0), (0, 255, 0));
        assert_eq!(wall_color(3, 0), (0, 0, 255));
        assert_eq!(wall_color(4, 0), (255, 255, 0));
        assert_eq!(wall_color(5, 0), (255, 0, 255));
        assert_eq!(wall_color(42, 0), (255, 255, 255));
    }

    #[test]
    fn wall_color_is_dimmed_on_north_south_faces() {
        assert_eq!(wall_color(1, 1), (127, 0, 0));
        assert_eq!(wall_color(4, 1), (127, 127, 0));
    }
}
