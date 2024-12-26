use graphics::{types::Color, CharacterCache, DrawState, Rectangle, Text};
use opengl_graphics::GlGraphics;

use crate::FontManager;

pub struct ButtonState {
    is_visible: bool,
    is_hovered: bool,
}

impl ButtonState {
    fn new() -> Self {
        ButtonState {
            is_visible: true,
            is_hovered: false,
        }
    }
}

pub struct ButtonProps {
    pub typ: ButtonType,
    pub font: String,
    pub font_size: u32,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub border_radius: f64,
    pub bg_color: Color,
    pub bg_hover_color: Color,
    pub text_color: Color,
    pub text_hover_color: Color,
}

#[derive(Clone, Copy)]
pub enum ButtonType {
    Continue,
    NewGame,
    Settings,
    Exit,
}

pub struct Button {
    typ: ButtonType,
    props: ButtonProps,
    state: ButtonState,

    text: Text,
    rect: Rectangle,
}

impl Button {
    pub fn new(props: ButtonProps) -> Self {
        let text = Text::new_color(props.text_color, props.font_size);
        let rect = Rectangle::new_round(props.bg_color, props.border_radius);

        Button {
            typ: props.typ,
            text,
            rect,
            props,
            state: ButtonState::new(),
        }
    }

    pub fn get_type(&self) -> ButtonType {
        self.typ
    }

    pub fn render(
        &mut self,
        font_manager: &mut FontManager,
        context: &graphics::Context,
        gl: &mut GlGraphics,
    ) {
        if !self.state.is_visible {
            return;
        }

        let bg_color = match self.state.is_hovered {
            true => self.props.bg_hover_color,
            false => self.props.bg_color,
        };

        let text_color = match self.state.is_hovered {
            true => self.props.text_hover_color,
            false => self.props.text_color,
        };

        let box_position = [
            self.props.x,
            self.props.y,
            self.props.width,
            self.props.height,
        ];
        // self.rect.color = bg_color;
        self.rect
            .color(bg_color)
            .draw(box_position, &DrawState::default(), context.transform, gl);

        self.text.color = text_color;
        let cache = font_manager.get_cache(&self.props.font.as_str()).unwrap();
        let text_to_render = self.props.text.as_str();

        let mut text_render_width: f64 = 0.0;
        let mut text_render_height: f64 = self.props.font_size as f64;
        for ch in text_to_render.chars() {
            let character = cache.character(self.props.font_size, ch).unwrap();
            text_render_height += character.advance_height();
            text_render_width += character.advance_width();
        }

        let center_of_box_position = [
            (self.props.x + self.props.width)
                - (self.props.width / 2.0)
                - (text_render_width / 2.0),
            (self.props.y + self.props.height) - (self.props.height / 2.0)
                + (text_render_height / 2.0) * 0.8,
        ];

        self.text
            .draw_pos(
                text_to_render,
                center_of_box_position,
                cache,
                &DrawState::default(),
                context.transform,
                gl,
            )
            .unwrap();
    }

    pub fn is_in_bounds(&self, pos: [f64; 2]) -> bool {
        let [x, y] = pos;

        x >= self.props.x
            && y >= self.props.y
            && x < self.props.x + self.props.width
            && y < self.props.y + self.props.height
    }

    // pub fn on_click(&self, context: &LevelGameStateContext) {}
}
