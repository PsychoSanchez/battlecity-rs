extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::cmp;
use std::path::Path;

mod animation;
mod constants;
mod game;
mod pickup;
mod player;
mod projectile;
mod transform;
mod wall;

use constants::*;
use game::Game;
use glutin_window::GlutinWindow as Window;
use graphics::rectangle::square;
use graphics::{clear, DrawState, Image, Rectangle, Transformed};
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, ButtonEvent, Key, ResizeEvent};
use player::Player;
use wall::generate_walls;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
                    // texture: Texture,
                    // image: Image, // Render object
                    // players: Vec<Player>,
                    // walls: Vec<Vec<wall::Wall>>,
                    // pickups: Vec<Rectangle>,
                    // bullets: Vec<Rectangle>,
                    // animations: Vec<Rectangle>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Image
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        // const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // let square = rectangle::square(0.0, 0.0, 50.0);
        // let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                // .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            // rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }

    fn process_input(&mut self, args: &ButtonArgs) {
        let is_pressed = args.state == piston::ButtonState::Press;

        if !is_pressed {
            return;
        }

        // for player in &mut players {
        //     if let Button::Keyboard(btn) = args.button {
        //         let (movement, fire) = player.get_controls();
        //         // capture pressed and hold before update

        //         if btn.eq(&fire) {
        //             player.shoot();
        //         }

        //         if btn.eq(&movement[0]) {
        //             player.move_up();
        //         } else if btn.eq(&movement[1]) {
        //             player.move_right();
        //         } else if btn.eq(&movement[2]) {
        //             player.move_down();
        //         } else if btn.eq(&movement[3]) {
        //             player.move_left();
        //         }

        //         // let (up, right, down, left) =
        //         //     (movement[0], movement[1], movement[2], movement[3]);

        //         // match btn {
        //         //     up => player.move_up(),
        //         //     right => player.move_right(),
        //         //     down => player.move_down(),
        //         //     left => player.move_left(),
        //         //     fire => player.shoot(),
        //         //     _ => (),
        //         // }
        //     }
        // }
    }
}

fn load_game_textures() -> Texture {
    let settings = TextureSettings::new();
    let texture = Texture::from_path(Path::new("resources/tanks.png"), &settings).unwrap();

    texture
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let column_count = 30;
    let row_count = 20;
    let default_cell_size = 32.0;
    let window_size = [
        column_count as f64 * default_cell_size,
        row_count as f64 * default_cell_size,
    ];
    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Battle City", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(false)
        .build()
        .unwrap();

    let texture = load_game_textures();

    // let walls = generate_walls(column_count, row_count);
    // let mut players = vec![
    //     Player::new(
    //         0,
    //         [0, 0],
    //         [Key::Up, Key::Right, Key::Down, Key::Left],
    //         Key::Space,
    //     )
    //     .set_tiles(TANK_1_TILES),
    //     Player::new(
    //         1,
    //         [column_count - 1, row_count - 1],
    //         [Key::W, Key::D, Key::S, Key::A],
    //         Key::X,
    //     )
    //     .set_tiles(TANK_2_TILES),
    // ];

    // let mut brick_image =
    //     Image::new().src_rect([0.0 * TILE_SIZE, 0.0 * TILE_SIZE, TILE_SIZE, TILE_SIZE]);
    // let mut concrete_image =
    //     Image::new().src_rect([1.0 * TILE_SIZE, 0.0 * TILE_SIZE, TILE_SIZE, TILE_SIZE]);
    // let mut tank_image =
    //     Image::new().src_rect([0.0 * TILE_SIZE, 1.0 * TILE_SIZE, TILE_SIZE, TILE_SIZE]);

    // let image = Image::new();

    let mut game = Game::new(GlGraphics::new(opengl), texture, column_count, row_count);
    game.set_window_size(window_size);

    // Create a new game and run it.
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     rotation: 0.0,
    // };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.resize_args() {
            game.set_window_size(args.window_size);
            // window_size = args.window_size;
        }

        if let Some(args) = e.render_args() {
            game.render(&args);
            // app.gl.draw(args.viewport(), |c, gl| {
            //     //Clear the screen
            //     clear([0.0, 0.0, 0.0, 1.0], gl);

            //     let [window_width, window_height] = window_size;
            //     // get min cell size
            //     let cell_size = f64::min(
            //         window_width / column_count as f64,
            //         window_height / row_count as f64,
            //     );

            //     // center the board
            //     let board_width = cell_size * column_count as f64;
            //     let board_height = cell_size * row_count as f64;
            //     let x_offset = (window_width - board_width) / 2.0;
            //     let y_offset = (window_height - board_height) / 2.0;

            //     for player in &players {
            //         let src_rect = player.get_frame();
            //         let [x, y] = player.get_position();

            //         let target_rect = [
            //             x_offset + cell_size * x as f64,
            //             y_offset + cell_size * y as f64,
            //             cell_size,
            //             cell_size,
            //         ];

            //         image.src_rect(*src_rect).rect(target_rect).draw(
            //             &texture,
            //             &DrawState::default(),
            //             c.transform,
            //             gl,
            //         );
            //     }

            //     for row in 0..row_count {
            //         for column in 0..column_count {
            //             let wall = &walls[row as usize][column as usize];
            //             let src_rect = wall.get_frame();

            //             let target_rect = [
            //                 x_offset + cell_size * column as f64,
            //                 y_offset + cell_size * row as f64,
            //                 cell_size,
            //                 cell_size,
            //             ];

            //             image.src_rect(*src_rect).rect(target_rect).draw(
            //                 &texture,
            //                 &DrawState::default(),
            //                 c.transform,
            //                 gl,
            //             );
            //         }
            //     }
            // });
        }

        if let Some(args) = e.update_args() {
            // app.update(&args);
            game.update(&args);
        }

        if let Some(args) = e.button_args() {
            game.process_input(&args);

            // let is_pressed = args.state == piston::ButtonState::Press;
            // if is_pressed {
            //     for player in &mut players {
            //         if let Button::Keyboard(btn) = args.button {
            //             player.on_press(btn);
            //             // let (movement, fire) = player.get_controls();
            //             // capture pressed and hold before update

            //             // if btn.eq(&fire) {
            //             //     player.shoot();
            //             // }

            //             // if btn.eq(&movement[0]) {
            //             //     player.move_up();
            //             // } else if btn.eq(&movement[1]) {
            //             //     player.move_right();
            //             // } else if btn.eq(&movement[2]) {
            //             //     player.move_down();
            //             // } else if btn.eq(&movement[3]) {
            //             //     player.move_left();
            //             // }

            //             // let (up, right, down, left) =
            //             //     (movement[0], movement[1], movement[2], movement[3]);

            //             // match btn {
            //             //     up => player.move_up(),
            //             //     right => player.move_right(),
            //             //     down => player.move_down(),
            //             //     left => player.move_left(),
            //             //     fire => player.shoot(),
            //             //     _ => (),
            //             // }
            //         }
            //     }
            // }
        }
    }
}
