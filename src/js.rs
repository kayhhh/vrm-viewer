use bevy::prelude::*;
use bevy_blob_loader::{path::serialize_url, BlobLoaderPlugin};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/init.js")]
extern "C" {
    fn init();
    fn update() -> Option<String>;
}

pub struct JsPlugin;

impl Plugin for JsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlobLoaderPlugin)
            .add_systems(Startup, init_js)
            .add_systems(Update, update_js);
    }
}

fn init_js() {
    init();
}

fn update_js(asset_server: Res<AssetServer>, mut vrm_asset: ResMut<crate::plugin::VrmAsset>) {
    if let Some(blob_url) = update() {
        let url = serialize_url(&blob_url, "vrm");
        vrm_asset.0 = asset_server.load(url);
    }
}
