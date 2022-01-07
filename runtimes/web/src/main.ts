import { WasmProgram } from "./wasm/load";
import { Renderer } from "./rendering/renderer";
import { resizeCanvasToDisplaySize } from "twgl.js";

const canvas = document.getElementById("canvas") as HTMLCanvasElement;

fetch("./manifest.json")
  .then((response) => response.json())
  .then((json) => start(json));

const start = async (manifest: ManifestV1) => {
  if (manifest.manifestVersion == null) {
    console.log(manifest);
    throw new Error("Manifest version not specified");
  }
  if (manifest.manifestVersion > 1) {
    console.log(manifest);
    throw new Error("Only manifest version 1 is currently supported");
  }

  const renderer = new Renderer(
    canvas,
    manifest.fonts,
    manifest.palette,
    manifest.postProcessing === false ? false : true
  );
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

  let frameCount = 0;
  let t: number | undefined;
  const frame = (tNext?: number) => {
    if (paused) {
      requestAnimationFrame(frame);
      return;
    }

    let dt;
    if (t == null || tNext == null) {
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

    frameCount += 1;
    requestAnimationFrame(frame);
  };
  frame();

  canvas.addEventListener("dblclick", () => {
    canvas.requestFullscreen();
  });
};
