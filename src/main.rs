use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::*;

mod plugin;
use plugin::VrmViewerPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VrmViewerPlugin))
        .run();
}

#[wasm_bindgen(start)]
fn start() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            VrmViewerPlugin,
        ))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .run();
}
