import { FontSpriteProgram } from "./fontsprite/fontsprite";
import * as twgl from "twgl.js";

type Fonts = {
  narrow: string;
  square: string;
};

export class Renderer {
  state: {
    gl: WebGLRenderingContext;
    fontSpriteProgram: FontSpriteProgram;
    narrowFontTexture: WebGLTexture;
    squareFontTexture: WebGLTexture;
  };

  constructor(canvas: HTMLCanvasElement, fonts: Fonts) {
    const gl = canvas.getContext("webgl");
    if (gl == null) {
      throw new Error("Could not start webgl");
    }

    const narrowFontTexture = twgl.createTexture(gl, {
      src: fonts.narrow,
      min: gl.NEAREST,
      mag: gl.NEAREST,
    });
    const squareFontTexture = twgl.createTexture(gl, {
      src: fonts.square,
      min: gl.NEAREST,
      mag: gl.NEAREST,
    });

    this.state = {
      gl,
      fontSpriteProgram: new FontSpriteProgram(gl),
      narrowFontTexture,
      squareFontTexture,
    };
  }

  clear() {
    const { gl } = this.state;
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  render() {
    this.clear();
    this.state.fontSpriteProgram.render(
      this.state.narrowFontTexture,
      this.state.squareFontTexture
    );
  }
}
