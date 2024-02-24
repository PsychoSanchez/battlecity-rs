pub struct Animation {
    position: [f64; 2],
    frames: Vec<[f64; 4]>,
    current_frame: usize,
    frame_duration: f64,
    frame_dt: f64,
}

impl Animation {
    pub fn new(position: [f64; 2], frames: Vec<[f64; 4]>, frame_duration: f64) -> Animation {
        Animation {
            position,
            frames,
            current_frame: 0,
            frame_duration,
            frame_dt: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.frame_dt += dt;

        if self.frame_dt >= self.frame_duration {
            self.frame_dt = 0.0;
            self.current_frame = self.current_frame + 1 % self.frames.len();
        }
    }

    pub fn get_frame(&self) -> &[f64; 4] {
        &self.frames[self.current_frame]
    }

    pub fn get_position(&self) -> &[f64; 2] {
        &self.position
    }
}
