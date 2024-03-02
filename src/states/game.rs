use bevy::prelude::*;
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
        app.add_systems(OnEnter(AppState::Game), init_game);
        //app.add_systems(OnEnter(AppState::Game), load_level);
    }
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
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    // Spawn floor.
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(50.0, 1.0, 50.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // Spawn player cam.
    commands.spawn(Camera3dBundle::default())
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2.0, 5.0, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}

// If space_editor magically updates to 0.13 before the end of the jam, bring
// this back.
//
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