use graphics::Image;
use opengl_graphics::{GlGraphics, Texture};
use piston::{Button, ButtonArgs, Key, RenderArgs, UpdateArgs};

use crate::{
    animation::Animation,
    constants::*,
    pickup::Pickup,
    player::Player,
    projectile::Projectile,
    wall::{self, generate_walls, Wall},
};

fn is_in_bounds(x: i32, y: i32, column_count: u8, row_count: u8) -> bool {
    x >= 0 && x < column_count as i32 && y >= 0 && y < row_count as i32
}

struct GameRender {
    column_count: u8,
    row_count: u8,
    image: Image,
    texture: Texture,
    cell_size: f64,
    x_offset: f64,
    y_offset: f64,
}

impl GameRender {
    pub fn set_window_size(&mut self, window_size: [f64; 2]) {
        let [window_width, window_height] = window_size;
        // get min cell size
        self.cell_size = f64::min(
            window_width / self.column_count as f64,
            window_height / self.row_count as f64,
        );

        // center the board
        let board_width = self.cell_size * self.column_count as f64;
        let board_height = self.cell_size * self.row_count as f64;
        self.x_offset = (window_width - board_width) / 2.0;
        self.y_offset = (window_height - board_height) / 2.0;
    }

    pub fn draw_image(
        &self,
        gl: &mut GlGraphics,
        c: &graphics::Context,
        frame: &[f64; 4],
        position: &[i32; 2],
    ) {
        let [x, y] = *position;
        let target = [
            self.x_offset + self.cell_size * x as f64,
            self.y_offset + self.cell_size * y as f64,
            self.cell_size,
            self.cell_size,
        ];

        self.image.src_rect(*frame).rect(target).draw(
            &self.texture,
            &graphics::DrawState::default(),
            c.transform,
            gl,
        );
    }
}

pub struct Game {
    gl: GlGraphics,
    column_count: u8,
    row_count: u8,
    // texture: Texture,
    // image: Image, // Render object
    players: Vec<Player>,
    walls: Vec<Vec<Wall>>,
    pickups: Vec<Pickup>,
    bullets: Vec<Projectile>,
    animations: Vec<Animation>,
    // window_size: [f64; 2],
    accumulated_time: f64,
    last_update: f64,
    update_interval: f64,
    // cell_size: f64,
    // x_offset: f64,
    // y_offset: f64,
    render: GameRender,
}

impl Game {
    pub fn new(gl: GlGraphics, texture: Texture, column_count: u8, row_count: u8) -> Game {
        Game {
            gl,
            // texture,
            column_count,
            row_count,
            // image: Image::new(),
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
            // window_size: [0.0, 0.0],
            last_update: 0.0,
            accumulated_time: 0.0,
            update_interval: 0.1,
            // cell_size: 0.0,
            // x_offset: 0.0,
            // y_offset: 0.0,
            render: GameRender {
                column_count,
                row_count,
                image: Image::new(),
                texture,
                cell_size: 0.0,
                x_offset: 0.0,
                y_offset: 0.0,
            },
        }
    }

    pub fn set_window_size(&mut self, window_size: [f64; 2]) {
        self.render.set_window_size(window_size);
        // self.window_size = window_size;

        // let [window_width, window_height] = self.window_size;
        // // get min cell size
        // self.cell_size = f64::min(
        //     window_width / self.column_count as f64,
        //     window_height / self.row_count as f64,
        // );

        // // center the board
        // let board_width = self.cell_size * self.column_count as f64;
        // let board_height = self.cell_size * self.row_count as f64;
        // self.x_offset = (window_width - board_width) / 2.0;
        // self.y_offset = (window_height - board_height) / 2.0;
    }

    // fn draw_image(
    //     &mut self,
    //     gl: &mut GlGraphics,
    //     c: &graphics::Context,
    //     frame: &[f64; 4],
    //     position: &[i32; 2],
    // ) {
    //     let [x, y] = *position;
    //     let target_rect = [
    //         self.x_offset + self.cell_size * x as f64,
    //         self.y_offset + self.cell_size * y as f64,
    //         self.cell_size,
    //         self.cell_size,
    //     ];

    //     self.image.src_rect(*frame).rect(target_rect).draw(
    //         &self.texture,
    //         &graphics::DrawState::default(),
    //         c.transform,
    //         gl,
    //     );
    // }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            //Clear the screen
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for player in &self.players {
                let frame = player.get_frame();
                let position = player.get_position();
                self.render.draw_image(gl, &c, frame, &position);

                // self.draw_image(gl, &c, frame, &position);

                // let target_rect = [
                //     x_offset + cell_size * x as f64,
                //     y_offset + cell_size * y as f64,
                //     cell_size,
                //     cell_size,
                // ];

                // self.image.src_rect(*src_rect).rect(target_rect).draw(
                //     &self.texture,
                //     &DrawState::default(),
                //     c.transform,
                //     gl,
                // );
            }

            self.walls
                .iter()
                .flat_map(|row| {
                    row.iter()
                        .filter(|wall| wall.variant() != wall::WallType::Empty)
                        .collect::<Vec<_>>()
                })
                .for_each(|wall| {
                    let frame = wall.get_frame();
                    let position = wall.get_position();
                    self.render.draw_image(gl, &c, frame, &position);
                });

            // for row in 0..self.row_count {
            //     for column in 0..self.column_count {
            //         let wall = &self.walls[row as usize][column as usize];
            //         let frame = wall.get_frame();
            //         let position = wall.get_position();
            //         self.render.draw_image(gl, &c, frame, &position);

            //         // let target_rect = [
            //         //     x_offset + cell_size * x as f64,
            //         //     y_offset + cell_size * y as f64,
            //         //     cell_size,
            //         //     cell_size,
            //         // ];

            //         // self.image.src_rect(*src_rect).rect(target_rect).draw(
            //         //     &self.texture,
            //         //     &DrawState::default(),
            //         //     c.transform,
            //         //     gl,
            //         // );
            //     }
            // }

            for pickup in &self.pickups {
                let frame = pickup.get_frame();
                let position = pickup.get_position();
                self.render.draw_image(gl, &c, frame, &position);

                // let target_rect = [
                //     x_offset + cell_size * x as f64,
                //     y_offset + cell_size * y as f64,
                //     cell_size,
                //     cell_size,
                // ];

                // self.image.src_rect(*src_rect).rect(target_rect).draw(
                //     &self.texture,
                //     &DrawState::default(),
                //     c.transform,
                //     gl,
                // );
            }

            for animation in &self.animations {
                let frame = animation.get_frame();
                let position = animation.get_position();
                self.render.draw_image(gl, &c, frame, position);

                // let target_rect = [
                //     x_offset + cell_size * x,
                //     y_offset + cell_size * y,
                //     cell_size,
                //     cell_size,
                // ];

                // self.image.src_rect(*src_rect).rect(target_rect).draw(
                //     &self.texture,
                //     &DrawState::default(),
                //     c.transform,
                //     gl,
                // );
            }

            for bullet in &self.bullets {
                let frame = bullet.get_frame();
                let position = bullet.get_position();
                self.render.draw_image(gl, &c, frame, &position);

                // let target_rect = [
                //     x_offset + cell_size * x as f64,
                //     y_offset + cell_size * y as f64,
                //     cell_size,
                //     cell_size,
                // ];

                // self.image.src_rect(*src_rect).rect(target_rect).draw(
                //     &self.texture,
                //     &DrawState::default(),
                //     c.transform,
                //     gl,
                // );
            }
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
