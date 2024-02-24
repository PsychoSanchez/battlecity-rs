use opengl_graphics::{GlGraphics, Texture};
use piston::{Button, ButtonArgs, Key, RenderArgs, UpdateArgs};

use crate::{
    animation::Animation,
    constants::*,
    pickup::Pickup,
    player::Player,
    projectile::Projectile,
    render::{GameRender, GameRenderObject},
    wall::{self, generate_walls, Wall},
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
                    [Key::Up, Key::Right, Key::Down, Key::Left],
                    Key::Space,
                )
                .set_tiles(TANK_1_TILES),
                Player::new(
                    1,
                    [column_count as i32 - 1, row_count as i32 - 1],
                    [Key::W, Key::D, Key::S, Key::A],
                    Key::X,
                )
                .set_tiles(TANK_2_TILES),
            ],
            walls: generate_walls(column_count, row_count),
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

            for player in &self.players {
                self.render.draw(gl, &c, player);
            }

            self.walls
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .filter(|wall| wall.variant() != wall::WallType::Empty)
                        .collect::<Vec<_>>()
                })
                .for_each(|wall| self.render.draw(gl, &c, wall));

            for pickup in &self.pickups {
                self.render.draw(gl, &c, pickup);
            }

            for animation in &self.animations {
                self.render.draw(gl, &c, animation);
            }

            for bullet in &self.bullets {
                self.render.draw(gl, &c, bullet)
            }

            // self.bullets
            //     .iter()
            //     .for_each(|bullet| self.render.draw(gl, &c, bullet));
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.accumulated_time += args.dt;

        if self.accumulated_time - self.last_update < self.update_interval {
            return;
        }

        self.last_update = self.accumulated_time;

        for player in &mut self.players {
            player.update(args);

            if let Some(direction) = player.get_pressed_direction() {
                let position = player.get_position();
                let [x, y] = direction.position_from(&position);

                if is_in_bounds(x, y, self.column_count, self.row_count) {
                    let wall = &self.walls[y as usize][x as usize];
                    if !wall.is_solid() {
                        player.set_position([x, y]);
                    }
                }
                player.set_direction(direction);
            }

            if player.get_is_fire_pressed() && player.shoot() {
                let position = player.get_position();
                let direction = player.get_direction();
                let bullet = Projectile::new(position, *direction);
                self.bullets.push(bullet);
            }
        }

        for bullet in &mut self.bullets {
            // bullet.update(args);
            let [x, y] = bullet.get_direction().position_from(&bullet.get_position());

            if !is_in_bounds(x, y, self.column_count, self.row_count) {
                // self.bullets.retain(|b| b != bullet);
                continue;
            }

            let wall = &mut self.walls[y as usize][x as usize];
            if wall.is_solid() {
                wall.damage();
                // self.bullets.retain(|b| b != bullet);
                continue;
            }

            for player in &mut self.players {
                if player.get_position() == [x, y] {
                    // self.bullets.retain(|b| b != bullet);
                    continue;
                }
            }

            bullet.set_position([x, y]);
        }

        // for animation in &mut self.animations {
        //     animation.update(args.dt);
        // }
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
