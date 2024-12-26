use crate::{
    core::level::{Level, LevelDrawContext, LevelGameStateContext},
    game::Game,
    GameEvent,
};

pub struct GameLevel {
    game: Game,
}

impl GameLevel {
    pub fn new(column_count: u8, row_count: u8, window_size: [f64; 2]) -> GameLevel {
        let mut game = Game::new(column_count, row_count);
        game.set_window_size(window_size);
        GameLevel { game }
    }
}
impl Level for GameLevel {
    fn resize(&mut self, size: [f64; 2]) {
        self.game.set_window_size(size)
    }

    fn render(&mut self, context: LevelDrawContext) {
        let LevelDrawContext { context, gl, .. } = context;
        self.game.render(context, gl)
    }

    fn update(&mut self, args: &piston::UpdateArgs) {
        self.game.update(args)
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
        context.events.push_back(GameEvent::ChangeLevel(0))
    }

    fn on_input(&mut self, args: &piston::ButtonArgs) {
        self.game.process_input(args)
    }

    fn on_focus(&mut self, args: &piston::TouchArgs) {
        todo!()
    }

    fn on_blur(&mut self, args: &piston::TouchArgs) {
        todo!()
    }
}
