const to_load = [];

export function init(load_url) {
  document.body.addEventListener("drop", (e) => {
    e.preventDefault();
    e.stopPropagation();

    const file = e.dataTransfer.files[0];
    const url = URL.createObjectURL(file);
    console.log("Dropped file: " + url);

    to_load.push(url);
  });
}

export function update() {
  return to_load.shift();
}
