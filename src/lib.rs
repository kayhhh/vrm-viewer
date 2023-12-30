use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_vrm::{Vrm, VrmBundle, VrmPlugin};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins((DefaultPlugins, VrmViewerPlugin))
        .run();
}

pub struct VrmViewerPlugin;

impl Plugin for VrmViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VrmPlugin)
            .init_resource::<VrmAsset>()
            .add_systems(Startup, setup)
            .add_systems(Update, (drag_and_drop, set_vrm, rotate_vrm));
    }
}

#[derive(Resource, Default)]
struct VrmAsset(pub Handle<Vrm>);

#[derive(Component)]
struct VrmTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut vrm_asset: ResMut<VrmAsset>) {
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
    vrm_asset.0 = vrm.clone();

    commands.spawn((
        VrmTag,
        VrmBundle {
            vrm,
            scene_bundle: SceneBundle {
                transform,
                ..default()
            },
        },
    ));
}

fn set_vrm(mut vrm: Query<&mut Handle<Vrm>, With<VrmTag>>, vrm_asset: ResMut<VrmAsset>) {
    for mut handle in vrm.iter_mut() {
        if *handle != vrm_asset.0 {
            info!("Setting new VRM: {:?}", vrm_asset.0.path());
            *handle = vrm_asset.0.clone();
        }
    }
}

fn rotate_vrm(time: Res<Time>, mut query: Query<&mut Transform, With<VrmTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() / 3.0));
    }
}

fn drag_and_drop(
    mut events: EventReader<FileDragAndDrop>,
    asset_server: Res<AssetServer>,
    mut vrm_asset: ResMut<VrmAsset>,
) {
    for event in events.read() {
        if let FileDragAndDrop::DroppedFile {
            path_buf,
            window: _,
        } = event
        {
            info!("Dropped file: {:?}", path_buf);
            let path = String::from(path_buf.to_str().unwrap());
            vrm_asset.0 = asset_server.load(path);
        }
    }
}
