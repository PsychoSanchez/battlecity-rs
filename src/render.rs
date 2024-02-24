use graphics::Image;
use opengl_graphics::{GlGraphics, Texture};

pub struct GameRender {
    column_count: u8,
    row_count: u8,
    image: Image,
    texture: Texture,
    cell_size: f64,
    x_offset: f64,
    y_offset: f64,
}

pub trait GameRenderObject {
    fn is_visible(&self) -> bool;
    fn get_position(&self) -> &[i32; 2];
    fn get_frame(&self) -> &[f64; 4];
}

impl GameRender {
    pub fn new(column_count: u8, row_count: u8, texture: Texture) -> GameRender {
        GameRender {
            column_count,
            row_count,
            texture,
            image: Image::new(),
            cell_size: 0.0,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }

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

    pub fn draw(&self, gl: &mut GlGraphics, c: &graphics::Context, obj: &dyn GameRenderObject) {
        if !obj.is_visible() {
            return;
        }

        let [x, y] = *obj.get_position();
        let target = [
            self.x_offset + self.cell_size * x as f64,
            self.y_offset + self.cell_size * y as f64,
            self.cell_size,
            self.cell_size,
        ];

        self.image.src_rect(*obj.get_frame()).rect(target).draw(
            &self.texture,
            &graphics::DrawState::default(),
            c.transform,
            gl,
        );
    }
}
