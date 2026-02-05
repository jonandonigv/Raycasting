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

    pub fn move_forward(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x + self.dir_x * move_speed;
        let new_y = self.pos_y + self.dir_y * move_speed;

        if world_map[new_y as usize][new_x as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    pub fn move_backward(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x - self.dir_x * move_speed;
        let new_y = self.pos_y - self.dir_y * move_speed;

        if world_map[new_y as usize][new_x as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    pub fn strafe_left(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x - self.plane_x * move_speed;
        let new_y = self.pos_y - self.plane_y * move_speed;

        if world_map[new_y as usize][new_x as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
    }

    pub fn strafe_right(&mut self, world_map: &Vec<Vec<i32>>, move_speed: f32) {
        let new_x = self.pos_x + self.plane_x * move_speed;
        let new_y = self.pos_y + self.plane_y * move_speed;

        if world_map[new_y as usize][new_x as usize] == 0 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }
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
}
