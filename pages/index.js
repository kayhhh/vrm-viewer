import init from "./wasm/vrm_viewer.js";

// Prevent default drag and drop behavior
document.body.addEventListener("dragover", (e) => {
  e.preventDefault();
});

document.body.addEventListener("drop", (e) => {
  e.preventDefault();
});

// Start Bevy
console.log("Initializing WASM");

init().then(() => {
  console.log("WASM initialized");
});
