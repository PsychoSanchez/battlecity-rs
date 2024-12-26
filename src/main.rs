extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod animation;
mod constants;
mod core;
mod game;
mod levels;
mod pickup;
mod player;
mod projectile;
mod render;
mod transform;
mod wall;

use core::level::{Level, LevelDrawContext, LevelGameStateContext};
use core::ui::{Button, ButtonProps, ButtonType};
use std::collections::VecDeque;
use std::str::FromStr;

use constants::*;
use game::Game;
use glutin_window::GlutinWindow as Window;
use graphics::clear;
use graphics::color::{BLACK, RED, WHITE};
use levels::game::GameLevel;
use levels::menu::MenuLevel;
use levels::GameScenes;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{ButtonEvent, MouseCursorEvent, PressEvent, ResizeEvent};

struct FontCache<'a> {
    path: &'a str,
    cache: GlyphCache<'a>,
}

impl FontCache<'_> {
    fn new(font: &str) -> FontCache {
        let texture_settings = TextureSettings::new().filter(Filter::Linear);
        FontCache {
            path: font,
            cache: GlyphCache::new(font, (), texture_settings)
                .expect(&format!("failed to load font `{}`", font)),
        }
    }
}

struct FontManager {
    fonts: Vec<FontCache<'static>>,
}

impl FontManager {
    fn new() -> Self {
        FontManager { fonts: vec![] }
    }

    fn load(&mut self, font: &'static str) {
        self.fonts.push(FontCache::new(font));
    }

    fn get_cache(&mut self, font: &str) -> Option<&mut GlyphCache<'static>> {
        self.fonts.iter_mut().find_map(|c| {
            if c.path == font {
                Some(&mut c.cache)
            } else {
                None
            }
        })
    }
}

struct LevelSelector {
    levels: [GameScenes; 2],
    active_level_index: usize,
}

impl LevelSelector {
    fn new(column_count: u8, row_count: u8, window_size: [f64; 2]) -> LevelSelector {
        LevelSelector {
            levels: [
                GameScenes::MenuLevel(MenuLevel::new()),
                GameScenes::GameLevel(GameLevel::new(column_count, row_count, window_size)),
            ],
            active_level_index: 0,
        }
    }

    fn get_current_level(&mut self) -> &mut dyn Level {
        let active_lvl = self.levels.get_mut(self.active_level_index).unwrap();
        // .unwrap_or(&mut self.levels[0]);
        // let first_lvl = self.levels.get_mut(0);
        // let active_level = match lvl {
        //     Some(mut lvl) => lvl,
        //     None => first_lvl.unwrap(),
        // };

        match active_lvl {
            GameScenes::GameLevel(l) => l,
            GameScenes::MenuLevel(l) => l,
            GameScenes::SettingsLevel(l) => l,
        }
    }

    fn change_level(&mut self, index: usize) {
        if index < self.levels.len() {
            self.active_level_index = index;
        }
    }
}

struct GameSettings {
    window_size: [f64; 2],
    column_count: usize,
    row_count: usize,
    cell_size: usize,
}

enum GameEvent {
    StartNewGame,
    ContinueLastGame,
    ChangeLevel(usize),
    ChangeSettings(GameSettings),
    Exit,
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let column_count = COLUMN_COUNT;
    let row_count = ROW_COUNT;
    let default_cell_size = DEFAULT_CELL_SIZE;
    let window_size = [
        column_count as f64 * default_cell_size,
        row_count as f64 * default_cell_size + SCOREBOARD_HEIGHT * 2.0,
    ];

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Battle City", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut font_manager = FontManager::new();
    font_manager.load("resources/Verdana.ttf");

    let mut level_selector = LevelSelector::new(column_count, row_count, window_size);
    let mut gl = GlGraphics::new(opengl);
    let mut mouse_position: [f64; 2] = [-1.0, -1.0];
    let mut window_events = Events::new(EventSettings::new());
    let mut game_state = LevelGameStateContext {
        events: &mut VecDeque::<GameEvent>::new(),
    };

    'main_loop: while let Some(e) = window_events.next(&mut window) {
        while let Some(e) = game_state.events.pop_front() {
            match e {
                GameEvent::StartNewGame => todo!(),
                GameEvent::ContinueLastGame => level_selector.change_level(1),
                GameEvent::ChangeSettings(_) => todo!(),
                GameEvent::ChangeLevel(level_index) => level_selector.change_level(level_index),
                GameEvent::Exit => break 'main_loop,
            }
        }

        let level = level_selector.get_current_level();
        if let Some(args) = e.resize_args() {
            level.resize(args.window_size);
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |context, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);
                level.render(LevelDrawContext {
                    font_manager: &mut font_manager,
                    context: &context,
                    gl,
                });
            });
        }

        if let Some(args) = e.update_args() {
            level.update(&args);
        }

        if let Some(args) = e.button_args() {
            level.on_input(&args);
        }

        e.mouse_cursor(|pos| {
            mouse_position = pos;
            true
        });

        if let Some(args) = e.press_args() {
            match args {
                piston::Button::Keyboard(_) => (),
                piston::Button::Mouse(mouse_btn) => match mouse_btn {
                    piston::MouseButton::Unknown => todo!(),
                    piston::MouseButton::Left => {
                        level.on_press(&args, mouse_position, &mut game_state);
                    }
                    piston::MouseButton::Right => {
                        level_selector.change_level((level_selector.active_level_index + 1) % 2)
                    }
                    piston::MouseButton::Middle => todo!(),
                    piston::MouseButton::X1 => todo!(),
                    piston::MouseButton::X2 => todo!(),
                    piston::MouseButton::Button6 => todo!(),
                    piston::MouseButton::Button7 => todo!(),
                    piston::MouseButton::Button8 => todo!(),
                },
                piston::Button::Controller(_) => todo!(),
                piston::Button::Hat(_) => todo!(),
            }
        }
    }
}
