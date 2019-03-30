import init, {
  CanvasRenderingContext2dRenderer
} from "./pkg/rust_canvas_example.js";
import JavaScriptRenderer from "./javascript_renderer.js";

const RADIUS = 10;

const setMode = mode => {
  const node = document.getElementById("mode");
  node.replaceChild(document.createTextNode(mode), node.firstChild);
};

const setFPS = elapsed => {
  const node = document.getElementById("fps");
  node.replaceChild(document.createTextNode(30000 / elapsed), node.firstChild);
};

let renderer;
let last;
let i = 0;
const update = () => {
  renderer.update();
  window.requestAnimationFrame(now => {
    i += 1;
    if (i == 30) {
      setFPS(now - last);
      last = now;
      i = 0;
    }
    update();
  });
};

//
// MAIN
//

const circles_count = document.getElementById("circles-count");
circles_count.addEventListener("change", ev => {
  renderer.setCirclesCount(Number(ev.target.value));
});

const canvas = document.querySelector("canvas");

switch (location.search) {
  case "?Rust":
    setMode("Rust + CanvasRenderingContext2D");
    init("./pkg/rust_canvas_example_bg.wasm").then(() => {
      renderer = new CanvasRenderingContext2dRenderer(
        canvas.getContext("2d"),
        canvas.width,
        canvas.height,
        RADIUS
      );
      renderer.setCirclesCount(Number(circles_count.value));
      update();
    });
    break;

  default:
    setMode("JavaScript + CanvasRenderingContext2D");
    renderer = new JavaScriptRenderer(
      canvas.getContext("2d"),
      canvas.width,
      canvas.height,
      RADIUS
    );
    renderer.setCirclesCount(Number(circles_count.value));
    update();
    break;
}

// vim: set ts=2 sw=2 et:
