import { FontSpriteProgram } from "./fontsprite/fontsprite";

export class Renderer {
  state: {
    gl: WebGLRenderingContext;
    fontSpriteProgram: FontSpriteProgram;
  };

  constructor(canvas: HTMLCanvasElement, fontTexture: string) {
    const gl = canvas.getContext("webgl");
    if (gl == null) {
      throw new Error("Could not start webgl");
    }

    this.state = {
      gl,
      fontSpriteProgram: new FontSpriteProgram(gl, fontTexture),
    };
  }

  clear() {
    const { gl } = this.state;
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  render() {
    this.clear();
    this.state.fontSpriteProgram.render();
  }
}
