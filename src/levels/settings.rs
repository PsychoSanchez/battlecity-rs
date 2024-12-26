use crate::core::level::{Level, LevelDrawContext, LevelGameStateContext};

pub struct SettingsLevel {}

impl Level for SettingsLevel {
    fn resize(&mut self, size: [f64; 2]) {
        // todo!()
    }

    fn render(&mut self, context: LevelDrawContext) {
        todo!()
    }

    fn update(&mut self, args: &piston::UpdateArgs) {
        todo!()
    }

    fn on_touch(&mut self, args: &piston::TouchArgs) {
        todo!()
    }

    fn on_press(
        &mut self,
        args: &piston::Button,
        pos: [f64; 2],
        context: &mut LevelGameStateContext,
    ) {
        todo!()
    }

    fn on_input(&mut self, args: &piston::ButtonArgs) {
        todo!()
    }

    fn on_focus(&mut self, args: &piston::TouchArgs) {
        todo!()
    }

    fn on_blur(&mut self, args: &piston::TouchArgs) {
        todo!()
    }
}
