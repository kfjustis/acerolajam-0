pub mod init;
pub mod game;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    Focus, // Force users to click into the game on web.
    MainMenu,
    #[default]
    Game
}