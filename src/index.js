const CONFIG = {
  width: 1000,
  height: 1000,
  radius: 10,
  numberOfCircles: 1000
};

const canvas = document.getElementById("canvas");
const context = canvas.getContext("2d");
const fps = document.getElementById("fps");

let renderer;
let requestId;
let elapsed;
let prev;

const start = (now = null) => {
  if (now) {
    elapsed = now - prev;
    prev = now;
    setFps(1000 / elapsed);
  }
  renderer.update();
  renderer.render(context);
  requestId = window.requestAnimationFrame(start);
};

const stop = () => window.cancelAnimationFrame(requestId);

const setFps = value => (fps.textContent = value);

const modeSelect = document.getElementById("mode");
modeSelect.onchange = event => {
  stop();

  const mode = event.target.value;
  switch (mode) {
    case "js-arcfill":
      import("./js-arcfill.js").then(({ Renderer }) => {
        renderer = new Renderer(CONFIG);
      });
      break;
    case "rust-arcfill":
      import("../pkg/index.js").then(({ ArcFillRenderer }) => {
        renderer = new ArcFillRenderer(
          CONFIG.width,
          CONFIG.height,
          CONFIG.radius,
          CONFIG.numberOfCircles
        );
      });
      break;
    default:
      return;
  }

  start();
};

import("./js-arcfill.js").then(({ Renderer }) => {
  renderer = new Renderer(CONFIG);
  start();
});

// vim: set ts=2 sw=2 et:
