use crate::{
    transform::LookDirection, SHELL_DOWN_TILE, SHELL_LEFT_TILE, SHELL_RIGHT_TILE, SHELL_UP_TILE,
};

pub struct Projectile {
    position: [[i32; 2]; 2],
    direction: LookDirection,
}

impl Projectile {
    pub fn new(position: [i32; 2], direction: LookDirection) -> Projectile {
        Projectile {
            position: [position, position],
            direction,
        }
    }

    pub fn get_position(&self) -> [i32; 2] {
        self.position[0]
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position[1] = self.position[0];
        self.position[0] = position;
    }

    pub fn get_direction(&self) -> &LookDirection {
        &self.direction
    }

    pub fn get_frame(&self) -> &[f64; 4] {
        match self.direction {
            LookDirection::Up => &SHELL_UP_TILE,
            LookDirection::Down => &SHELL_DOWN_TILE,
            LookDirection::Left => &SHELL_LEFT_TILE,
            LookDirection::Right => &SHELL_RIGHT_TILE,
        }
    }
}
