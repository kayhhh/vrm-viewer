use bevy::prelude::*;
use vrm_viewer::VrmViewerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "pages/assets".to_string(),
                ..default()
            }),
            VrmViewerPlugin,
        ))
        .run();
}
