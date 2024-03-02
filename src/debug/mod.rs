
use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use smooth_bevy_cameras::controllers::fps::FpsCameraController;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugins(WorldInspectorPlugin::new());
            app.add_systems(Update, bevy::window::close_on_esc);
            app.add_systems(Update, set_cursor_look);
        }
    }
}

fn set_cursor_look(
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut controller: Query<&mut FpsCameraController>,
) {
    let mut primary_window = windows.single_mut();

    if keys.pressed(KeyCode::AltLeft) {
        // Unlock and show the cursor.
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;

        // Block camera movement.
        if controller.is_empty() {
            return;
        }
        let mut cam = controller.single_mut();
        cam.enabled = false;
    } else {
        // Lock and hide the cursor.
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;

        // Enable camera movement.
        if controller.is_empty() {
            return;
        }
        let mut cam = controller.single_mut();
        cam.enabled = true;
    }
}