use std::collections::VecDeque;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::{Button, ButtonArgs, TouchArgs, UpdateArgs};

use crate::{FontManager, GameEvent};

pub struct LevelDrawContext<'a> {
    pub font_manager: &'a mut FontManager,
    pub context: &'a Context,
    pub gl: &'a mut GlGraphics,
}

pub struct LevelGameStateContext<'a> {
    pub events: &'a mut VecDeque<GameEvent>,
}

pub trait Level {
    fn resize(&mut self, size: [f64; 2]);
    fn render(&mut self, context: LevelDrawContext);
    fn update(&mut self, args: &UpdateArgs);
    fn on_touch(&mut self, args: &TouchArgs);
    fn on_press(&mut self, args: &Button, pos: [f64; 2], context: &mut LevelGameStateContext);
    fn on_input(&mut self, args: &ButtonArgs);
    fn on_focus(&mut self, args: &TouchArgs);
    fn on_blur(&mut self, args: &TouchArgs);
}
