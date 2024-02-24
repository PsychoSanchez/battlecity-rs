pub const TILE_SIZE: f64 = 16.0;
pub const BRICK_TILE: [f64; 4] = [
    0.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const CONCRETE_TILE: [f64; 4] = [
    1.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMOR_PICKUP_TILE: [f64; 4] = [
    2.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SHELL_UP_TILE: [f64; 4] = [
    3.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SHELL_RIGHT_TILE: [f64; 4] = [
    4.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SHELL_DOWN_TILE: [f64; 4] = [
    5.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SHELL_LEFT_TILE: [f64; 4] = [
    6.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const HEALTH_PICKUP_TILE: [f64; 4] = [
    7.0 * TILE_SIZE,
    0.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_1_TILE_UP: [f64; 4] = [
    0.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_2_TILE_UP: [f64; 4] = [
    1.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_1_TILE_RIGHT: [f64; 4] = [
    2.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_2_TILE_RIGHT: [f64; 4] = [
    3.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_1_TILE_DOWN: [f64; 4] = [
    4.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_2_TILE_DOWN: [f64; 4] = [
    5.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_1_TILE_LEFT: [f64; 4] = [
    6.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_2_TILE_LEFT: [f64; 4] = [
    7.0 * TILE_SIZE,
    1.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_3_TILE_UP: [f64; 4] = [
    0.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_4_TILE_UP: [f64; 4] = [
    1.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_3_TILE_RIGHT: [f64; 4] = [
    2.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_4_TILE_RIGHT: [f64; 4] = [
    3.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_3_TILE_DOWN: [f64; 4] = [
    4.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_4_TILE_DOWN: [f64; 4] = [
    5.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_3_TILE_LEFT: [f64; 4] = [
    6.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const TANK_4_TILE_LEFT: [f64; 4] = [
    7.0 * TILE_SIZE,
    2.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const EXPLOSION_FRAME_1_TILE: [f64; 4] = [
    0.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const EXPLOSION_FRAME_2_TILE: [f64; 4] = [
    1.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const EXPLOSION_FRAME_3_TILE: [f64; 4] = [
    2.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const EMPTY_FRAME_TILE: [f64; 4] = [
    3.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SPAWN_FRAME_1_TILE: [f64; 4] = [
    4.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SPAWN_FRAME_2_TILE: [f64; 4] = [
    5.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const SPAWN_FRAME_3_TILE: [f64; 4] = [
    6.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const NET_TILE: [f64; 4] = [
    7.0 * TILE_SIZE,
    3.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_1_TILE_UP: [f64; 4] = [
    0.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_2_TILE_UP: [f64; 4] = [
    1.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_1_TILE_RIGHT: [f64; 4] = [
    2.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_2_TILE_RIGHT: [f64; 4] = [
    3.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_1_TILE_DOWN: [f64; 4] = [
    4.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_2_TILE_DOWN: [f64; 4] = [
    5.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_1_TILE_LEFT: [f64; 4] = [
    6.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_2_TILE_LEFT: [f64; 4] = [
    7.0 * TILE_SIZE,
    4.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_3_TILE_UP: [f64; 4] = [
    0.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_4_TILE_UP: [f64; 4] = [
    1.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_3_TILE_RIGHT: [f64; 4] = [
    2.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_4_TILE_RIGHT: [f64; 4] = [
    3.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_3_TILE_DOWN: [f64; 4] = [
    4.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_4_TILE_DOWN: [f64; 4] = [
    5.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_3_TILE_LEFT: [f64; 4] = [
    6.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];
pub const ARMORED_TANK_4_TILE_LEFT: [f64; 4] = [
    7.0 * TILE_SIZE,
    5.0 * TILE_SIZE,
    TILE_SIZE - 0.5,
    TILE_SIZE - 0.5,
];

pub const TANK_1_TILES: [[f64; 4]; 8] = [
    TANK_1_TILE_UP,
    TANK_1_TILE_RIGHT,
    TANK_1_TILE_DOWN,
    TANK_1_TILE_LEFT,
    ARMORED_TANK_1_TILE_UP,
    ARMORED_TANK_1_TILE_RIGHT,
    ARMORED_TANK_1_TILE_DOWN,
    ARMORED_TANK_1_TILE_LEFT,
];
pub const TANK_2_TILES: [[f64; 4]; 8] = [
    TANK_2_TILE_UP,
    TANK_2_TILE_RIGHT,
    TANK_2_TILE_DOWN,
    TANK_2_TILE_LEFT,
    ARMORED_TANK_2_TILE_UP,
    ARMORED_TANK_2_TILE_RIGHT,
    ARMORED_TANK_2_TILE_DOWN,
    ARMORED_TANK_2_TILE_LEFT,
];
pub const TANK_3_TILES: [[f64; 4]; 8] = [
    TANK_3_TILE_UP,
    TANK_3_TILE_RIGHT,
    TANK_3_TILE_DOWN,
    TANK_3_TILE_LEFT,
    ARMORED_TANK_3_TILE_UP,
    ARMORED_TANK_3_TILE_RIGHT,
    ARMORED_TANK_3_TILE_DOWN,
    ARMORED_TANK_3_TILE_LEFT,
];
pub const TANK_4_TILES: [[f64; 4]; 8] = [
    TANK_4_TILE_UP,
    TANK_4_TILE_RIGHT,
    TANK_4_TILE_DOWN,
    TANK_4_TILE_LEFT,
    ARMORED_TANK_4_TILE_UP,
    ARMORED_TANK_4_TILE_RIGHT,
    ARMORED_TANK_4_TILE_DOWN,
    ARMORED_TANK_4_TILE_LEFT,
];
