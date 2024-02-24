use piston::{input::Key, UpdateArgs};

use crate::{render::GameRenderObject, transform::LookDirection, TANK_1_TILES};

pub struct Player {
    id: u32,
    // [current, previous]
    position: [[i32; 2]; 2],

    lives: u32,
    health: u32,
    armor: u32,

    kills: u32,

    is_alive: bool,
    spawn: [i32; 2],
    spawn_health: u32,
    spawn_armor: u32,

    last_shot_dt: f64,
    shot_interval: f64,

    movement_controls: [Key; 4],
    movement_controls_state: [bool; 4],
    fire_control: Key,
    fire_control_state: bool,

    direction: LookDirection,

    tiles: [[f64; 4]; 8],
}

impl GameRenderObject for Player {
    fn is_visible(&self) -> bool {
        self.health > 0
    }

    fn get_frame(&self) -> &[f64; 4] {
        self.get_frame()
    }

    fn get_position(&self) -> &[i32; 2] {
        &self.position[0]
    }

    fn get_previous_position(&self) -> &[i32; 2] {
        &self.position[1]
    }
}

impl Player {
    pub fn new(id: u32, spawn: [i32; 2], movement_controls: [Key; 4], fire_control: Key) -> Player {
        Player {
            id,
            position: [spawn, spawn],
            lives: 3,
            health: 3,
            armor: 0,
            kills: 0,
            is_alive: true,
            spawn,
            spawn_health: 100,
            spawn_armor: 0,
            last_shot_dt: 0.0,
            shot_interval: 0.5,
            movement_controls,
            movement_controls_state: [false; 4],
            fire_control,
            fire_control_state: false,
            direction: LookDirection::Up,
            tiles: TANK_1_TILES,
        }
    }

    pub fn set_tiles(mut self, tiles: [[f64; 4]; 8]) -> Self {
        self.tiles = tiles;

        self
    }

    pub fn get_tiles(&self) -> &[[f64; 4]; 8] {
        &self.tiles
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.last_shot_dt += args.dt;
    }

    pub fn shoot(&mut self) -> bool {
        if self.is_reloading() {
            return false;
        }

        self.last_shot_dt = 0.0;

        return true;
    }

    pub fn is_reloading(&self) -> bool {
        self.last_shot_dt < self.shot_interval
    }

    pub fn get_direction(&self) -> &LookDirection {
        &self.direction
    }

    pub fn set_direction(&mut self, direction: LookDirection) {
        self.direction = direction;
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position[1] = self.position[0];
        self.position[0] = position;
    }

    pub fn get_position(&self) -> [i32; 2] {
        self.position[0]
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    // Returns bool: is killed
    pub fn damage(&mut self) -> bool {
        if self.armor > 0 {
            self.armor -= 1;
        } else {
            self.health -= 1;
        }

        if self.health == 0 {
            self.is_alive = false;
        }

        !self.is_alive
    }

    pub fn get_is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn inc_kill_count(&mut self) {
        self.kills += 1;
    }

    pub fn respawn(&mut self) {
        self.position[0] = self.spawn;
        self.position[1] = self.spawn;
        self.health = self.spawn_health;
        self.armor = self.spawn_armor;
        self.is_alive = true;
    }

    pub fn on_press(&mut self, key: Key) {
        self.movement_controls
            .iter()
            .position(|k| key.eq(&k))
            .map(|i| {
                self.movement_controls_state[i] = true;
            });

        if key.eq(&self.fire_control) {
            self.fire_control_state = true;
        }
    }

    pub fn on_release(&mut self, key: Key) {
        self.movement_controls
            .iter()
            .position(|k| key.eq(&k))
            .map(|i| {
                self.movement_controls_state[i] = false;
            });

        if key.eq(&self.fire_control) {
            self.fire_control_state = false;
        }
    }

    pub fn get_pressed_direction(&self) -> Option<LookDirection> {
        self.movement_controls_state
            .iter()
            .position(|&b| b)
            .map(|i| match i {
                0 => LookDirection::Up,
                1 => LookDirection::Right,
                2 => LookDirection::Down,
                3 => LookDirection::Left,
                _ => unreachable!(),
            })
    }

    pub fn get_is_fire_pressed(&self) -> bool {
        self.fire_control_state
    }

    pub fn get_frame(&self) -> &[f64; 4] {
        let shift = if self.armor > 0 { 4 } else { 0 };

        match self.direction {
            LookDirection::Up => &self.tiles[0 + shift],
            LookDirection::Right => &self.tiles[1 + shift],
            LookDirection::Down => &self.tiles[2 + shift],
            LookDirection::Left => &self.tiles[3 + shift],
        }
    }
}
