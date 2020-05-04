import { WasmProgram } from "./wasm/load";
import { Renderer } from "./rendering/renderer";

const pauseButton = document.getElementById("pause") as HTMLButtonElement;
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const debugEl = document.getElementById("debug") as HTMLPreElement;

const manifest: ManifestV1 = {
  manifestVersion: 1,
  program: "./wasm_minimal.wasm",
  fonts: {
    gridSize: {
      width: 8,
      height: 16,
    },
    narrow: "./text-font.png",
    square: "./text-font-square.png",
  },
  inputMappings: [
    { address: 0, comment: "Move Up", keys: ["ArrowUp", "w"] },
    { address: 1, comment: "Move Down", keys: ["ArrowDown", "s"] },
    { address: 2, comment: "Move Left", keys: ["ArrowLeft", "a"] },
    { address: 3, comment: "Move Right", keys: ["ArrowRight", "d"] },
    { address: 5, comment: "Accept", keys: ["Enter", " "] },
    { address: 6, comment: "Cancel", keys: ["Escape"] },
    { address: 7, comment: "Shift", keys: ["Shift"] },
  ],
};

const start = async (manifest: ManifestV1) => {
  if (manifest.version > 1) {
    throw new Error("Only manifest version 1 is currently supported");
  }

  const renderer = new Renderer(canvas, {
    narrow: manifest.fonts.narrow,
    square: manifest.fonts.square,
  });
  const program = await WasmProgram.load(manifest.program);
  let paused = false;

  document.addEventListener("keydown", (e) => {
    manifest.inputMappings.forEach((mapping) => {
      if (mapping.keys.includes(e.key)) {
        program.setInput(mapping.address, 1);
      }
    });
    e.key;
    console.log(e.key);
  });

  document.addEventListener("keyup", (e) => {
    manifest.inputMappings.forEach((mapping) => {
      if (mapping.keys.includes(e.key)) {
        program.setInput(mapping.address, 0);
      }
    });
    e.key;
  });

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
      x: config.cols * manifest.fonts.gridSize.width,
      y: config.rows * manifest.fonts.gridSize.height,
    };

    renderer.render(virtualScreenSize);

    const end = performance.now();
    debug.time = end - start + "ms";
    debugEl.innerText = JSON.stringify(debug, undefined, 2);

    requestAnimationFrame(frame);
  };
  frame();
};

start(manifest);
