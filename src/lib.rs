use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_vrm::{VrmBundle, VrmPlugin};
use wasm_bindgen::prelude::*;

/// Start the bevy app
#[wasm_bindgen]
pub fn start() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../assets".to_string(),
                ..default()
            }),
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_vrm)
        .run();
}

#[derive(Component)]
struct VrmTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    commands.spawn((
        VrmBundle {
            vrm: asset_server.load("default_398.vrm"),
            scene_bundle: SceneBundle {
                transform,
                ..default()
            },
        },
        VrmTag,
    ));

    commands.spawn((Camera3dBundle::default(), bevy_vrm::mtoon::MtoonMainCamera));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, -5.0),
            ..default()
        },
        bevy_vrm::mtoon::MtoonSun,
    ));
}

fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<VrmTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 3.0));
    }
}
