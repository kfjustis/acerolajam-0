use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

use crate::states::AppState;

pub struct InitStatePlugin;

impl Plugin for InitStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Acerola Jam 0 | kftoons".into(),
                        name: Some("kftoons.app.bevy".into()),
                        resolution: (960., 540.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Prevent wasm from overriding F5, Ctrl+R, etc.
                        prevent_default_event_handling: false,
                        window_theme: Some(WindowTheme::Dark),
                        visible: true,
                        ..default()
                    }),
                    ..default()
                }
            ));
        app.init_state::<AppState>();
    }
}