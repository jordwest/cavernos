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
    virtualScreen: twgl.FramebufferInfo;
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
    const virtualScreen = twgl.createFramebufferInfo(gl, [
      {
        min: gl.NEAREST,
        mag: gl.NEAREST,
        width: 1,
        height: 1,
      },
    ]);

    this.state = {
      gl,
      fontSpriteProgram: new FontSpriteProgram(gl),
      narrowFontTexture,
      squareFontTexture,
      virtualScreen,
    };
  }

  clear() {
    const { gl } = this.state;
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  useRealScreen() {
    twgl.bindFramebufferInfo(this.state.gl, null);
    this.clear();
  }

  render(virtualScreenSize: Vec2) {
    const { gl } = this.state;

    twgl.resizeCanvasToDisplaySize(
      gl.canvas as HTMLCanvasElement,
      window.devicePixelRatio || 1
    );

    this.useRealScreen();
    // Render the virtual screen first
    //gl.viewport(0, 0, virtualScreenSize.x, virtualScreenSize.y);
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
    this.state.fontSpriteProgram.render(
      this.state.narrowFontTexture,
      this.state.squareFontTexture
    );
  }
}
