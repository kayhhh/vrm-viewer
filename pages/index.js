import init from "./wasm/vrm_viewer.js";

console.log("Initializing WASM");

init().then(() => {
  console.log("WASM initialized");
};
