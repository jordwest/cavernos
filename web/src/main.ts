import { WasmProgram } from "./wasm/load";
import { Renderer } from "./rendering/renderer";

declare var window: Window & {
  start?: (wasm: string) => void;
};

const pauseButton = document.getElementById("pause") as HTMLButtonElement;
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const debugEl = document.getElementById("debug") as HTMLPreElement;

// TODO: This should come from a configurable manifest
const GRID_SIZE = {
  x: 8,
  y: 16,
};

window.start = async (wasmSrc: string) => {
  const renderer = new Renderer(canvas, {
    narrow: "./text-font.png",
    square: "./text-font-square.png",
  });
  const program = await WasmProgram.load(wasmSrc);
  let paused = false;

  pauseButton.addEventListener("click", () => {
    paused = !paused;
    pauseButton.innerText = paused ? "Play" : "Pause";
  });

  const frame = (_t: number) => {
    let debug: object = {};

    if (paused) {
      requestAnimationFrame(frame);
      return;
    }

    const start = performance.now();

    program.tick();

    const config = program.config;
    const screen = program.screen;
    renderer.state.fontSpriteProgram.updateScreenTexture(
      config.cols,
      config.rows,
      screen
    );

    const virtualScreenSize = {
      x: config.cols * GRID_SIZE.x,
      y: config.rows * GRID_SIZE.y,
    };

    renderer.render(virtualScreenSize);

    const end = performance.now();
    debug.time = end - start + "ms";
    debugEl.innerText = JSON.stringify(debug, undefined, 2);

    requestAnimationFrame(frame);
  };
  frame();
};
