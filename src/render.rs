use std::path::Path;

use graphics::{DrawState, Image, Text};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, Texture, TextureSettings};

pub trait GameRenderObject {
    fn is_visible(&self) -> bool;
    fn get_position(&self) -> &[i32; 2];
    fn get_previous_position(&self) -> &[i32; 2];
    fn get_frame(&self) -> &[f64; 4];
}

pub struct GameRender {
    column_count: u8,
    row_count: u8,
    image: Image,
    texture: Texture,
    game_over_texture: Texture,
    cell_size: f64,
    x_offset: f64,
    y_offset: f64,
    scoreboard_height: f64,
    draw_state: DrawState,
    pub scoreboard: Scoreboard<'static>,
}
fn load_game_textures() -> Texture {
    let settings = TextureSettings::new()
        .compress(false)
        .filter(Filter::Nearest);

    let texture = Texture::from_path(Path::new("resources/tanks.png"), &settings).unwrap();

    texture
}

fn load_game_over_texture() -> Texture {
    let settings = TextureSettings::new()
        .compress(false)
        .filter(Filter::Nearest);

    let texture = Texture::from_path(Path::new("resources/gameover.png"), &settings).unwrap();

    texture
}

impl GameRender {
    pub fn new(column_count: u8, row_count: u8) -> GameRender {
        GameRender {
            column_count,
            row_count,
            texture: load_game_textures(),
            game_over_texture: load_game_over_texture(),
            image: Image::new(),
            cell_size: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
            scoreboard_height: 16.0,
            draw_state: DrawState::default(),
            scoreboard: Scoreboard::new(),
        }
    }

    pub fn set_window_size(&mut self, window_size: [f64; 2]) {
        let window_width = window_size[0];
        let window_height = window_size[1] - self.scoreboard_height * 2.0;
        // get min cell size
        self.cell_size = f64::min(
            window_width / self.column_count as f64,
            window_height / self.row_count as f64,
        );

        // center the board
        let board_width = self.cell_size * self.column_count as f64;
        let board_height = self.cell_size * self.row_count as f64;
        self.x_offset = (window_width - board_width) / 2.0;
        self.y_offset = self.scoreboard_height + (window_height - board_height) / 2.0;

        self.scoreboard.set_bounds([
            self.x_offset,
            0.0,
            self.x_offset + board_width,
            window_size[1],
        ]);
    }

    pub fn draw_game_over(&self, gl: &mut GlGraphics, c: &graphics::Context) {
        let target = [
            self.x_offset,
            self.y_offset,
            self.cell_size * self.column_count as f64,
            self.cell_size * self.row_count as f64,
        ];
        self.image
            .src_rect([0.0, 0.0, 124.0, 80.0])
            .rect(target)
            .draw(&self.game_over_texture, &self.draw_state, c.transform, gl);
    }

    pub fn draw(
        &self,
        gl: &mut GlGraphics,
        c: &graphics::Context,
        obj: &dyn GameRenderObject,
        interpolation: f64,
    ) {
        if !obj.is_visible() {
            return;
        }

        let [px, py] = *obj.get_previous_position();
        let [x, y] = *obj.get_position();
        let target_x = px as f64 + (x as f64 - px as f64) * interpolation;
        let target_y = py as f64 + (y as f64 - py as f64) * interpolation;
        let target = [
            self.x_offset + self.cell_size * target_x,
            self.y_offset + self.cell_size * target_y,
            self.cell_size,
            self.cell_size,
        ];

        self.image.src_rect(*obj.get_frame()).rect(target).draw(
            &self.texture,
            &self.draw_state,
            c.transform,
            gl,
        );
    }
}

pub struct Scoreboard<'a> {
    // textures: [Text; 4],
    texts: [String; 4],
    render_obj: Text,
    font: GlyphCache<'a>,
    bounds: [f64; 4],
}

pub struct ScoreboardPlayerState {
    pub kills: u32,
    pub lives: u32,
}

static FONT: &str = "resources/Verdana.ttf";

impl Scoreboard<'_> {
    pub fn new() -> Scoreboard<'static> {
        let font_size = 12;
        let render_obj = Text::new_color([1.0, 1.0, 1.0, 1.0], font_size);
        let texts = [String::new(), String::new(), String::new(), String::new()];
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let glyphs = GlyphCache::new(FONT, (), texture_settings)
            .expect(&format!("failed to load font `{}`", FONT));

        Scoreboard {
            texts,
            render_obj,
            font: glyphs,
            bounds: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn set_bounds(&mut self, bounds: [f64; 4]) {
        self.bounds = bounds;
    }

    pub fn set_score(&mut self, score: Vec<ScoreboardPlayerState>) {
        for i in 0..4 {
            self.texts[i] = match score.get(i) {
                Some(player) => format!(
                    "Player {} Kills: {} Lives: {}",
                    i + 1,
                    player.kills,
                    player.lives
                ),
                None => format!("Player {}: -", i + 1),
            };
        }
    }

    fn get_text_position(&self, index: usize) -> [f64; 2] {
        let [left, top, right, bottom] = self.bounds;
        match &index {
            0 => {
                let x = left + 4.0;
                let y = top + 14.0;
                [x, y]
            }
            3 => {
                let x = right - 170.0;
                let y = top + 14.0;
                [x, y]
            }
            2 => {
                let x = left + 4.0;
                let y = bottom - 2.0;
                [x, y]
            }
            1 => {
                let x = right - 170.0;
                let y = bottom - 4.0;
                [x, y]
            }
            _ => [0.0, 0.0],
        }
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, c: &graphics::Context) {
        for i in 0..4 {
            self.render_obj
                .draw_pos(
                    self.texts[i].as_str(),
                    self.get_text_position(i),
                    &mut self.font,
                    &DrawState::default(),
                    c.transform,
                    gl,
                )
                .expect("failed to draw text");
        }
    }
}
