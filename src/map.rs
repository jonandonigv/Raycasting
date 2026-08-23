use crate::constants::{MAP_HEIGHT, MAP_WIDTH};

pub type Map = [[i32; MAP_WIDTH]; MAP_HEIGHT];

pub const WORLD_MAP: Map = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ],
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
];

fn cell(map: &Map, x: i32, y: i32) -> Option<i32> {
    if x < 0 || y < 0 {
        return None;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return None;
    }
    Some(map[y][x])
}

pub fn is_wall(map: &Map, x: i32, y: i32) -> bool {
    cell(map, x, y).is_none_or(|value| value > 0)
}

pub fn wall_type(map: &Map, x: i32, y: i32) -> Option<i32> {
    let value = cell(map, x, y)?;
    (value > 0).then_some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_map_dimensions() {
        assert_eq!(WORLD_MAP.len(), MAP_HEIGHT);
        assert!(WORLD_MAP.iter().all(|row| row.len() == MAP_WIDTH));
    }

    #[test]
    fn border_is_fully_walled() {
        for x in 0..MAP_WIDTH {
            assert!(is_wall(&WORLD_MAP, x as i32, 0));
            assert!(is_wall(&WORLD_MAP, x as i32, MAP_HEIGHT as i32 - 1));
        }
        for y in 0..MAP_HEIGHT {
            assert!(is_wall(&WORLD_MAP, 0, y as i32));
            assert!(is_wall(&WORLD_MAP, MAP_WIDTH as i32 - 1, y as i32));
        }
    }

    #[test]
    fn all_values_are_valid_wall_types() {
        assert!(
            WORLD_MAP
                .iter()
                .flatten()
                .all(|&cell| (0..=5).contains(&cell))
        );
    }

    #[test]
    fn spawn_cell_is_empty() {
        assert!(!is_wall(&WORLD_MAP, 22, 12));
    }

    #[test]
    fn out_of_bounds_counts_as_wall() {
        assert!(is_wall(&WORLD_MAP, -1, 0));
        assert!(is_wall(&WORLD_MAP, 0, -1));
        assert!(is_wall(&WORLD_MAP, MAP_WIDTH as i32, 5));
        assert!(is_wall(&WORLD_MAP, 5, MAP_HEIGHT as i32));
    }

    #[test]
    fn wall_type_lookup() {
        assert_eq!(wall_type(&WORLD_MAP, 0, 0), Some(1));
        assert_eq!(wall_type(&WORLD_MAP, 22, 12), None);
        assert_eq!(wall_type(&WORLD_MAP, -1, 0), None);
        assert_eq!(wall_type(&WORLD_MAP, 5, MAP_HEIGHT as i32), None);
    }

    #[test]
    fn known_walls_have_expected_types() {
        assert_eq!(wall_type(&WORLD_MAP, 6, 4), Some(2));
        assert_eq!(wall_type(&WORLD_MAP, 15, 4), Some(3));
        assert_eq!(wall_type(&WORLD_MAP, 1, 16), Some(4));
        assert_eq!(wall_type(&WORLD_MAP, 6, 18), Some(5));
    }
}
