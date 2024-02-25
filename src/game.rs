use opengl_graphics::{GlGraphics, Texture};
use piston::{Button, ButtonArgs, Key, RenderArgs, UpdateArgs};

use crate::{
    animation::Animation,
    constants::*,
    pickup::{Pickup, PickupSpawnSystem, PickupType},
    player::Player,
    projectile::Projectile,
    render::{GameRender, GameRenderObject},
    transform::LookDirection,
    wall::{generate_walls, Wall, WallType},
};

fn is_in_bounds(x: i32, y: i32, column_count: u8, row_count: u8) -> bool {
    x >= 0 && x < column_count as i32 && y >= 0 && y < row_count as i32
}

pub struct Game {
    gl: GlGraphics,
    column_count: u8,
    row_count: u8,
    players: Vec<Player>,
    walls: Vec<Vec<Wall>>,
    pickups: Vec<Pickup>,
    pickup_spawn_systems: [PickupSpawnSystem; 2],
    max_pickups: usize,
    bullets: Vec<Projectile>,
    animations: Vec<Animation>,
    accumulated_time: f64,
    last_update: f64,
    update_interval: f64,
    render: GameRender,
}

impl Game {
    pub fn new(gl: GlGraphics, texture: Texture, column_count: u8, row_count: u8) -> Game {
        Game {
            gl,
            column_count,
            row_count,
            players: vec![
                Player::new(
                    0,
                    [0, 0],
                    LookDirection::Down,
                    [Key::Up, Key::Right, Key::Down, Key::Left],
                    Key::Space,
                )
                .set_tiles(TANK_1_TILES),
                Player::new(
                    1,
                    [column_count as i32 - 1, row_count as i32 - 1],
                    LookDirection::Up,
                    [Key::W, Key::D, Key::S, Key::A],
                    Key::X,
                )
                .set_tiles(TANK_3_TILES),
                Player::new(
                    2,
                    [0, row_count as i32 - 1],
                    LookDirection::Up,
                    [Key::T, Key::H, Key::G, Key::F],
                    Key::B,
                )
                .set_tiles(TANK_2_TILES),
                Player::new(
                    3,
                    [column_count as i32 - 1, 0],
                    LookDirection::Down,
                    [Key::I, Key::L, Key::K, Key::J],
                    Key::M,
                )
                .set_tiles(TANK_4_TILES),
            ],
            walls: generate_walls(column_count, row_count),
            pickup_spawn_systems: [
                PickupSpawnSystem::new(PickupType::Armor, 25.0),
                PickupSpawnSystem::new(PickupType::Health, 10.0),
            ],
            max_pickups: 5,
            pickups: vec![],
            bullets: vec![],
            animations: vec![],
            last_update: 0.0,
            accumulated_time: 0.0,
            update_interval: 0.1,
            render: GameRender::new(column_count, row_count, texture),
        }
    }

    pub fn set_window_size(&mut self, window_size: [f64; 2]) {
        self.render.set_window_size(window_size);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let lerp = f64::clamp(
                (self.accumulated_time - self.last_update) / self.update_interval,
                0.0,
                1.0,
            );

            for pickup in &self.pickups {
                self.render.draw(gl, &c, pickup, lerp);
            }

            for player in &self.players {
                self.render.draw(gl, &c, player, lerp);
            }

            self.walls
                .iter()
                .flat_map(|row| row.iter().filter(|wall| wall.variant() != WallType::Empty))
                .for_each(|wall| self.render.draw(gl, &c, wall, lerp));

            for animation in &self.animations {
                self.render.draw(gl, &c, animation, lerp);
            }

            for bullet in &self.bullets {
                self.render.draw(gl, &c, bullet, lerp)
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Frame tick logic
        self.accumulated_time += args.dt;

        self.animations.retain_mut(|animation| {
            animation.on_frame(args.dt);
            !animation.is_finished()
        });

        for player in &mut self.players {
            player.on_frame(args.dt);
        }

        for system in &mut self.pickup_spawn_systems {
            system.on_frame(args.dt);

            if self.pickups.len() >= self.max_pickups {
                continue;
            }

            if let Some(mut pickup) = system.get_pickup_to_spawn() {
                let empty_positions = self
                    .walls
                    .iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.iter().enumerate().filter_map(move |(x, wall)| {
                            if wall.variant() == WallType::Empty {
                                Some([x as i32, y as i32])
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<[i32; 2]>>();

                let spawn_position = empty_positions
                    .get(rand::random::<usize>() % empty_positions.len())
                    .unwrap();

                pickup.set_position(*spawn_position);

                self.pickups.push(pickup);

                system.reset_spawn_timer();
            }
        }

        // Game tick logic
        if self.accumulated_time - self.last_update < self.update_interval {
            return;
        }

        self.last_update = self.accumulated_time;

        for i in 0..self.players.len() {
            if !self.players[i].get_is_alive() {
                continue;
            }

            // Set player position to old position even if it's not moved to update animation state
            let player = &mut self.players[i];
            let position = player.get_position();
            player.set_position(position);

            if let Some(direction) = self.players[i].get_pressed_direction() {
                let position = self.players[i].get_position();
                let new_position = direction.position_from(&position);
                let [x, y] = new_position;

                let is_intersecting = !is_in_bounds(x, y, self.column_count, self.row_count)
                    || self.walls[y as usize][x as usize].is_solid()
                    || self.players[i..]
                        .iter()
                        .skip(1)
                        .filter(|p| p.get_is_alive())
                        .any(|p| {
                            let p_position = p.get_position();

                            p_position == new_position
                                || p_position == p.get_direction().position_from(&position)
                        });

                if !is_intersecting {
                    self.players[i].set_position([x, y]);
                }

                self.players[i].set_direction(direction);
            }

            if self.players[i].get_is_fire_pressed() && self.players[i].shoot() {
                let position = self.players[i].get_position();
                let direction = self.players[i].get_direction();
                let bullet = Projectile::new(self.players[i].get_id(), position, *direction);
                self.bullets.push(bullet);
            }
        }

        // Respawn dead players
        self.players
            .iter_mut()
            .filter(|player| !player.get_is_alive() && player.get_is_fire_pressed())
            .for_each(|player| {
                player.respawn();
                self.animations
                    .push(Animation::new_spawn(player.get_position()));
            });

        self.pickups.retain(|pickup| {
            let position = pickup.get_position();
            let mut players_to_pickup = self
                .players
                .iter_mut()
                .filter(|player| player.get_is_alive() && player.get_position() == *position);

            let is_picked_up = players_to_pickup.any(|player| match pickup.get_variant() {
                PickupType::Armor => player.add_armor(),
                PickupType::Health => player.add_health(),
            });

            !is_picked_up
        });

        self.update_bullets();
    }

    fn update_bullets(&mut self) {
        let bullets_length = self.bullets.len();
        let mut bullets_to_keep = vec![true; bullets_length];

        for i in 0..bullets_length {
            let bullet = &self.bullets[i];
            let position = bullet.get_position();
            let new_position = bullet.get_direction().position_from(&position);
            let [x, y] = new_position;

            // Compare against boundaries
            if !is_in_bounds(x, y, self.column_count, self.row_count) {
                bullets_to_keep[i] = false;
                continue;
            }

            // Compare against walls
            let wall = &mut self.walls[y as usize][x as usize];
            if wall.is_solid() {
                wall.damage();
                self.animations.push(Animation::new_explosion([x, y]));
                bullets_to_keep[i] = false;
                continue;
            }

            // Compare against players
            let players_to_damage = self
                .players
                .iter_mut()
                .filter(|player| player.get_is_alive() && player.get_position() == [x, y]);

            // let is_player_hit = players_to_damage.any(|_| true);
            // let is_player_hit = players_to_damage.count() > 0;
            let mut is_player_hit = false;
            let is_player_killed = players_to_damage
                .map(|player| {
                    is_player_hit = true;
                    player.damage()
                })
                .take(1)
                .any(|is_killed| is_killed);

            if is_player_killed {
                let kill_credit_player = self
                    .players
                    .iter_mut()
                    .find(|p| p.get_id() == bullet.get_owner_id())
                    .unwrap();

                kill_credit_player.inc_kill_count();
            }

            if is_player_hit {
                bullets_to_keep[i] = false;

                self.animations.push(Animation::new_explosion([x, y]));
                continue;
            }

            // Compare against other bullets
            self.bullets[i..]
                .iter()
                .skip(1)
                .map(|bullet| bullet.get_direction().position_from(&bullet.get_position()))
                .enumerate()
                .filter(|(_, position_b)| *position == *position_b || new_position == *position_b)
                .for_each(|(right_index, _)| {
                    bullets_to_keep[i] = false;
                    bullets_to_keep[i + right_index] = false;

                    self.animations.push(Animation::new_explosion(new_position));
                });
        }

        let mut iter = bullets_to_keep.iter();
        // Remove destroyed bullets
        self.bullets.retain(|_| *iter.next().unwrap());
        // Update bullets positions
        self.bullets.iter_mut().for_each(|bullet| {
            bullet.set_position(bullet.get_direction().position_from(&bullet.get_position()))
        });
    }

    pub fn process_input(&mut self, args: &ButtonArgs) {
        let is_pressed = args.state == piston::ButtonState::Press;
        for player in &mut self.players {
            if let Button::Keyboard(btn) = args.button {
                if is_pressed {
                    player.on_press(btn);
                } else {
                    player.on_release(btn);
                }
            }
        }
    }
}
