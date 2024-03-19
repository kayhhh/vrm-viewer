use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, vrm_viewer::VrmViewerPlugin))
        .run();
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
fn start() {
    use bevy::asset::AssetMetaCheck;
    use bevy_web_file_drop::WebFileDropPlugin;

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((WebFileDropPlugin, DefaultPlugins, VrmViewerPlugin))
        .run();
}
