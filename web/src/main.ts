import { WasmProgram } from "./wasm/load";
import { Renderer } from "./rendering/renderer";

declare var window: Window & {
  start?: (wasm: string) => void;
};

const pauseButton = document.getElementById("pause") as HTMLButtonElement;
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const debugEl = document.getElementById("debug") as HTMLPreElement;

window.start = async (wasmSrc: string) => {
  const renderer = new Renderer(canvas, "./text-font.png");
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

    const screen = program.screen;
    debug.x = screen.getUint8(50);

    renderer.render();

    const end = performance.now();
    debug.time = end - start + "ms";
    debugEl.innerText = JSON.stringify(debug, undefined, 2);

    requestAnimationFrame(frame);
  };
  frame();
};
