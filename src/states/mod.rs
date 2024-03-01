pub mod init;

use bevy::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    #[default]
    Focus, // Force users to click into the game on web.
    MainMenu,
    Game
}