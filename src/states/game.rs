use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

use crate::states::AppState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LookTransformPlugin);
        app.add_plugins(FpsCameraPlugin::default());
        app.add_systems(OnEnter(AppState::Game), (
            init_cursor,
            init_game));
    }
}

fn init_cursor(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = windows.single_mut();

    // Lock and hide the cursor.
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

fn init_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn light.
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 1_000_000.0 * 10.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    // Spawn floor.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(50.0, 1.0, 50.0)),
        material: materials.add(Color::rgb_u8(63, 33, 150)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Spawn player cam.
    commands.spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                enabled: true,
                translate_sensitivity: 20., // TODO Set to 0. after adding player physics.
                mouse_rotate_sensitivity: Vec2::splat(0.14),
                smoothing_weight: 0.45,
            },
            Vec3::new(0.0, 5., -25.), // Position (eye)
            Vec3::new(0., 5., 0.),    // Target (look)
            Vec3::Y,
        ));
}

// If space_editor magically updates to 0.13 before the end of the jam, bring
// this back.
//
// //app.add_systems(OnEnter(AppState::Game), load_level);
// const TEST_LEVEL_PATH: &str = "./maps/KFTOONS_testing.scn.ron";
// fn load_level(
//     mut commands: Commands, asset_server: Res<AssetServer>
// ) {
//     println!("should be loading...");
//     commands.spawn(DynamicSceneBundle {
//         // Scenes are loaded just like any other asset.
//         scene: asset_server.load(TEST_LEVEL_PATH),
//         ..default()
//     });
// }