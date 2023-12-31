import init from "./wasm/vrm_viewer.js";

async function run() {
  console.log("Initializing WASM");
  await init();
  console.log("WASM initialized");

  // Prevent default drag and drop behavior
  const canvas = document.querySelector("canvas");

  canvas.addEventListener("drop", (e) => {
    e.preventDefault();
    e.stopPropagation();
  });
}

run();
