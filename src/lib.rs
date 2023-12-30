use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_vrm::{VrmBundle, VrmPlugin};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StartOptions {
    pub assets_path: &'static str,
}

/// Start the bevy app
#[wasm_bindgen]
pub fn start(options: StartOptions) {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: String::from(options.assets_path),
                ..default()
            }),
            VrmPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_vrm, drag_and_drop))
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

fn drag_and_drop(mut events: EventReader<FileDragAndDrop>) {
    for event in events.read() {
        info!("{:?}", event);
    }
}
