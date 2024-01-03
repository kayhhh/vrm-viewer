use std::f32::consts::PI;

use bevy::{asset::AssetPath, prelude::*};
use bevy_vrm::{Vrm, VrmBundle, VrmPlugin};

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VrmPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, (read_dropped_files, rotate_vrm));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let mut transform = Transform::from_xyz(0.0, -1.0, -4.0);
    transform.rotate_y(PI);

    let vrm = asset_server.load("default_398.vrm");

    commands.spawn(VrmBundle {
        vrm,
        scene_bundle: SceneBundle {
            transform,
            ..default()
        },
    });
}

fn read_dropped_files(
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut vrms: Query<&mut Handle<Vrm>>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            let path = AssetPath::from_path(path_buf.as_path());
            info!("DroppedFile: {}", path);

            let mut vrm = vrms.single_mut();
            *vrm = asset_server.load(path);
        }
    }
}

fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<Handle<Vrm>>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 3.0));
    }
}
