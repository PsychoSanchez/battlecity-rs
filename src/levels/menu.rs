use std::str::FromStr;

use crate::{
    core::{
        level::{Level, LevelDrawContext, LevelGameStateContext},
        ui::{Button, ButtonProps, ButtonType},
    },
    GameEvent,
};
use graphics::color::{BLACK, RED, WHITE};

pub struct MenuLevel {
    buttons: Vec<Button>,
}
impl MenuLevel {
    pub fn new() -> MenuLevel {
        MenuLevel {
            buttons: vec![
                Button::new(ButtonProps {
                    typ: ButtonType::Continue,
                    font: String::from_str("resources/Verdana.ttf").unwrap(),
                    text: String::from_str("Continue").unwrap(),
                    x: 100.0,
                    y: 50.0,
                    width: 100.0,
                    height: 50.0,
                    font_size: 16,
                    text_color: BLACK,
                    bg_color: WHITE,
                    text_hover_color: WHITE,
                    bg_hover_color: BLACK,
                    border_radius: 5.0,
                }),
                Button::new(ButtonProps {
                    typ: ButtonType::NewGame,
                    font: String::from_str("resources/Verdana.ttf").unwrap(),
                    text: String::from_str("New Game").unwrap(),
                    x: 100.0,
                    y: 125.0,
                    width: 125.0,
                    height: 50.0,
                    font_size: 16,
                    text_color: RED,
                    text_hover_color: WHITE,
                    bg_color: WHITE,
                    bg_hover_color: BLACK,
                    border_radius: 5.0,
                }),
                Button::new(ButtonProps {
                    typ: ButtonType::Settings,
                    font: String::from_str("resources/Verdana.ttf").unwrap(),
                    text: String::from_str("Settings").unwrap(),
                    x: 100.0,
                    y: 200.0,
                    width: 100.0,
                    height: 50.0,
                    font_size: 16,
                    text_color: BLACK,
                    text_hover_color: WHITE,
                    bg_color: WHITE,
                    bg_hover_color: BLACK,
                    border_radius: 5.0,
                }),
                Button::new(ButtonProps {
                    typ: ButtonType::Exit,
                    font: String::from_str("resources/Verdana.ttf").unwrap(),
                    text: String::from_str("Exit").unwrap(),
                    x: 100.0,
                    y: 275.0,
                    width: 100.0,
                    height: 50.0,
                    font_size: 16,
                    text_color: BLACK,
                    text_hover_color: WHITE,
                    bg_color: WHITE,
                    bg_hover_color: BLACK,
                    border_radius: 5.0,
                }),
            ],
        }
    }
}
impl Level for MenuLevel {
    fn resize(&mut self, size: [f64; 2]) {
        // todo!()
    }

    fn render(&mut self, context: LevelDrawContext) {
        for button in self.buttons.iter_mut() {
            button.render(context.font_manager, context.context, context.gl);
        }
    }
    fn update(&mut self, args: &piston::UpdateArgs) {
        for button in self.buttons.iter_mut() {
            // button.update(args);
        }
    }

    fn on_touch(&mut self, args: &piston::TouchArgs) {
        for button in self.buttons.iter_mut() {
            // button.update(args);
        }
    }

    fn on_press(
        &mut self,
        args: &piston::Button,
        pos: [f64; 2],
        context: &mut LevelGameStateContext,
    ) {
        for button in self.buttons.iter_mut().filter(|btn| btn.is_in_bounds(pos)) {
            match button.get_type() {
                ButtonType::Continue => todo!(),
                ButtonType::NewGame => context.events.push_back(GameEvent::ChangeLevel(1)),
                ButtonType::Settings => todo!(),
                ButtonType::Exit => context.events.push_back(GameEvent::Exit),
            }
        }
    }

    fn on_input(&mut self, args: &piston::ButtonArgs) {
        // todo!()
    }

    fn on_focus(&mut self, args: &piston::TouchArgs) {
        todo!()
    }

    fn on_blur(&mut self, args: &piston::TouchArgs) {
        todo!()
    }
}
