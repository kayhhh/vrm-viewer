use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::*;

mod plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugin::VrmViewerPlugin))
        .run();
}

#[wasm_bindgen(start)]
fn start() {
    use bevy_web_file_drop::WebFileDropPlugin;

    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins((
            WebFileDropPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            plugin::VrmViewerPlugin,
        ))
        .run();
}
