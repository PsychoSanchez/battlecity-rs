extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod animation;
mod constants;
mod game;
mod pickup;
mod player;
mod projectile;
mod render;
mod transform;
mod wall;

use constants::*;
use game::Game;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, ResizeEvent};

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
        .vsync(true)
        .build()
        .unwrap();

    let mut game = Game::new(GlGraphics::new(opengl), column_count, row_count);
    game.set_window_size(window_size);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.resize_args() {
            game.set_window_size(args.window_size);
        }

        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(args) = e.button_args() {
            game.process_input(&args);
        }
    }
}
