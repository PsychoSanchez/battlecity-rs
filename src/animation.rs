use crate::{render::GameRenderObject, EXPLOSION_FRAMES, SPAWN_FRAMES};

pub struct Animation {
    position: [i32; 2],
    frames: Vec<[f64; 4]>,
    current_frame: usize,
    frame_duration: f64,
    frame_dt: f64,
}

impl GameRenderObject for Animation {
    fn is_visible(&self) -> bool {
        true
    }

    fn get_frame(&self) -> &[f64; 4] {
        &self.frames[self.current_frame]
    }

    fn get_position(&self) -> &[i32; 2] {
        &self.position
    }

    fn get_previous_position(&self) -> &[i32; 2] {
        &self.position
    }
}

impl Animation {
    pub fn new(position: [i32; 2], frames: Vec<[f64; 4]>, frame_duration: f64) -> Animation {
        Animation {
            position,
            frames,
            current_frame: 0,
            frame_duration,
            frame_dt: 0.0,
        }
    }

    pub fn new_explosion(position: [i32; 2]) -> Animation {
        Animation::new(position, EXPLOSION_FRAMES.to_vec(), 0.1)
    }

    pub fn new_spawn(position: [i32; 2]) -> Animation {
        Animation::new(position, SPAWN_FRAMES.to_vec(), 0.1)
    }

    pub fn on_frame(&mut self, dt: f64) {
        self.frame_dt += dt;

        if self.frame_dt >= self.frame_duration {
            self.frame_dt = 0.0;
            self.current_frame = self.current_frame + 1 % self.frames.len();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.current_frame == self.frames.len() - 1
    }
}
