use bevy::prelude::*;

mod states;

fn main() {
    App::new()
        .add_plugins(states::init::InitStatePlugin)
        .add_plugins(states::game::GameStatePlugin)
        .run()
}
