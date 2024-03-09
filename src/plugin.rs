use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::{mtoon::MtoonSun, Vrm, VrmBundle, VrmPlugin};

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PanOrbitCameraPlugin, VrmPlugin))
            .add_systems(Startup, setup)
            .add_systems(Update, read_dropped_files);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(1.0, 2.0, 5.0),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::new(0.0, 0.8, 0.0),
            ..default()
        },
        bevy_vrm::mtoon::MtoonMainCamera,
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 1000.0,
                ..default()
            },
            transform: Transform::from_xyz(2.0, 4.0, -5.0),
            ..default()
        },
        MtoonSun,
    ));

    let mut transform = Transform::default();
    transform.rotate_y(PI);

    commands.spawn(VrmBundle {
        scene_bundle: SceneBundle {
            transform,
            ..default()
        },
        vrm: asset_server.load("default_398.vrm"),
    });
}

fn read_dropped_files(
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut vrms: Query<&mut Handle<Vrm>>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = event {
            #[cfg(target_family = "wasm")]
            let path = String::from(path_buf.to_str().unwrap());
            #[cfg(not(target_family = "wasm"))]
            let path = bevy::asset::AssetPath::from_path(path_buf.as_path());

            info!("DroppedFile: {}", path);

            let mut vrm = vrms.single_mut();
            *vrm = asset_server.load(path);
        }
    }
}
