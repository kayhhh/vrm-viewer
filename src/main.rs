use vrm_viewer::StartOptions;

fn main() {
    vrm_viewer::start(StartOptions {
        assets_path: String::from("pages/assets"),
    });
}
