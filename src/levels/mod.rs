use game::GameLevel;
use menu::MenuLevel;
use settings::SettingsLevel;

pub mod game;
pub mod menu;
pub mod scoreboard;
pub mod settings;

pub enum GameScenes {
    MenuLevel(MenuLevel),
    GameLevel(GameLevel),
    SettingsLevel(SettingsLevel),
}
