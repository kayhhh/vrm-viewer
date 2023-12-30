{ lib, pkgs, system, build_inputs, native_build_inputs, makeRustPlatform }:
let
  rustBin = pkgs.rust-bin.stable.latest.default.override {
    targets = [ "wasm32-unknown-unknown" ];
  };

  rustPlatform = makeRustPlatform {
    cargo = rustBin;
    rustc = rustBin;
  };

  common = {
    version = "0.0.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

    buildInputs = build_inputs;
    nativeBuildInputs = native_build_inputs;

    LD_LIBRARY_PATH = lib.makeLibraryPath build_inputs;
  };
in {
  wasm = rustPlatform.buildRustPackage (common // {
    pname = "vrm-viewer";
    buildPhase = ''
      cargo build --target wasm32-unknown-unknown --profile wasm-release
    '';
    installPhase = ''
      mkdir -p $out/pages/wasm
      wasm-bindgen $out/target/wasm32-unknown-unknown/wasm-release/vrm_viewer.wasm --out-dir $out/pages/wasm --no-typescript
    '';
  });
}
