use bevy::prelude::*;

mod debug;
mod states;

fn main() {
    App::new()
        .add_plugins(states::init::InitStatePlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(states::game::GameStatePlugin)
        .run()
}
