import { WasmProgram } from "./wasm/load";
import { Renderer } from "./rendering/renderer";
import { resizeCanvasToDisplaySize } from "twgl.js";

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
  palette: "./palette.png",
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

  const renderer = new Renderer(canvas, manifest.fonts, manifest.palette);
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

  let frameCount = 0;
  let t: number | undefined;
  const frame = (tNext: number) => {
    let debug: object = {};

    if (paused) {
      requestAnimationFrame(frame);
      return;
    }

    const start = performance.now();

    let dt;
    if (t == null) {
      dt = 0.016;
    } else {
      dt = (tNext - t) / 1000;
    }
    t = tNext;

    program.tick(dt);

    let canvasResized = false;
    // Minimise DOM access by checking only every 10 frames
    if (frameCount % 10 === 0) {
      canvasResized = resizeCanvasToDisplaySize(
        canvas,
        window.devicePixelRatio || 1
      );
    }

    const config = program.config;
    const screen = program.screen;
    const bgColors = program.bgColors;
    const fgColors = program.fgColors;

    if (fgColors.update) {
      renderer.state.fgColorTable.updateWithColorData(
        config.cols,
        config.rows,
        fgColors.cells
      );
    }

    if (bgColors.update) {
      renderer.state.bgColorTable.updateWithColorData(
        config.cols,
        config.rows,
        bgColors.cells
      );
    }

    if (screen.update) {
      renderer.state.charsTable.updateWithCharData(
        config.cols,
        config.rows,
        screen.cells
      );
    }

    const virtualScreenSize = {
      x: config.cols * manifest.fonts.gridSize.width,
      y: config.rows * manifest.fonts.gridSize.height,
    };

    if (screen.update || bgColors.update || fgColors.update || canvasResized) {
      renderer.render(virtualScreenSize, config.rows, config.cols);
      renderer.state.gl.finish();
      fgColors.update = false;
      bgColors.update = false;
      screen.update = false;
    }

    // This DOM update actually causes the most memory allocations,
    // so only do it every 20 frames
    if (frameCount % 20 === 0) {
      const end = performance.now();
      debug.time = Math.round(end - start) + "ms";
      debugEl.textContent = JSON.stringify(debug, undefined, 2);
    }

    frameCount += 1;
    requestAnimationFrame(frame);
  };
  frame();
};

start(manifest);
