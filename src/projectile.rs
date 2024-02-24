use crate::{
    render::GameRenderObject, transform::LookDirection, SHELL_DOWN_TILE, SHELL_LEFT_TILE,
    SHELL_RIGHT_TILE, SHELL_UP_TILE,
};

pub struct Projectile {
    owner_id: u32,
    position: [[i32; 2]; 2],
    direction: LookDirection,
}

impl GameRenderObject for Projectile {
    fn is_visible(&self) -> bool {
        true
    }

    fn get_frame(&self) -> &[f64; 4] {
        match self.direction {
            LookDirection::Up => &SHELL_UP_TILE,
            LookDirection::Down => &SHELL_DOWN_TILE,
            LookDirection::Left => &SHELL_LEFT_TILE,
            LookDirection::Right => &SHELL_RIGHT_TILE,
        }
    }

    fn get_position(&self) -> &[i32; 2] {
        &self.position[0]
    }
    fn get_previous_position(&self) -> &[i32; 2] {
        &self.position[1]
    }
}

impl Projectile {
    pub fn new(owner_id: u32, position: [i32; 2], direction: LookDirection) -> Projectile {
        Projectile {
            owner_id,
            position: [position, position],
            direction,
        }
    }

    pub fn get_owner_id(&self) -> u32 {
        self.owner_id
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position[1] = self.position[0];
        self.position[0] = position;
    }

    pub fn get_direction(&self) -> &LookDirection {
        &self.direction
    }
}
