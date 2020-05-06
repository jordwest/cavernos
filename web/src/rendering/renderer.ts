import { FontSpriteProgram } from "./fontsprite/fontsprite";
import * as twgl from "twgl.js";
import { VirtualScreenProgram } from "./virtualscreen/virtualscreen";
import { Vec2 } from "../util/vec2";
import { LookupTable } from "./lookup_table";

const VIRTUAL_SCREEN_ATT = [
  {
    min: WebGLRenderingContext.NEAREST,
    mag: WebGLRenderingContext.NEAREST,
  },
];

type Fonts = {
  narrow: string;
  square: string;
};

export class Renderer {
  state: {
    gl: WebGLRenderingContext;
    fontSpriteProgram: FontSpriteProgram;
    virtualScreenProgram: VirtualScreenProgram;
    narrowFontTexture: WebGLTexture;
    squareFontTexture: WebGLTexture;
    palette: WebGLTexture;
    charsTable: LookupTable;
    bgColorTable: LookupTable;
    fgColorTable: LookupTable;
    virtualScreen: twgl.FramebufferInfo;
  };

  constructor(canvas: HTMLCanvasElement, fonts: Fonts, paletteSrc: string) {
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
    const palette = twgl.createTexture(gl, {
      src: paletteSrc,
      min: gl.NEAREST,
      mag: gl.NEAREST,
    });
    const virtualScreen = twgl.createFramebufferInfo(
      gl,
      VIRTUAL_SCREEN_ATT,
      1,
      1
    );

    this.state = {
      gl,
      fontSpriteProgram: new FontSpriteProgram(gl),
      virtualScreenProgram: new VirtualScreenProgram(gl),
      charsTable: new LookupTable(gl),
      bgColorTable: new LookupTable(gl),
      fgColorTable: new LookupTable(gl),
      narrowFontTexture,
      squareFontTexture,
      virtualScreen,
      palette,
    };
  }

  clear() {
    const { gl } = this.state;
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
  }

  useVirtualScreen() {
    twgl.bindFramebufferInfo(this.state.gl, this.state.virtualScreen);
    this.clear();
  }

  useRealScreen() {
    const { gl } = this.state;
    twgl.bindFramebufferInfo(gl, null);
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
    this.clear();
  }

  render(virtualScreenSize: Vec2, rows: number, cols: number) {
    const { gl } = this.state;

    if (
      this.state.virtualScreen.width != virtualScreenSize.x ||
      this.state.virtualScreen.height != virtualScreenSize.y
    ) {
      twgl.resizeFramebufferInfo(
        gl,
        this.state.virtualScreen,
        VIRTUAL_SCREEN_ATT,
        virtualScreenSize.x,
        virtualScreenSize.y
      );
    }

    // Render the virtual screen first
    this.useVirtualScreen();
    gl.viewport(0, 0, virtualScreenSize.x, virtualScreenSize.y);
    this.state.fontSpriteProgram.render({
      rows,
      cols,
      narrowFont: this.state.narrowFontTexture,
      squareFont: this.state.squareFontTexture,
      palette: this.state.palette,
      charsTable: this.state.charsTable.texture,
      backgroundColorTable: this.state.bgColorTable.texture,
      foregroundColorTable: this.state.fgColorTable.texture,
    });

    this.useRealScreen();
    // How many virtual screens can we fit into the real display area?
    // (Rounded down so we only ever have integer multiples - other multiples
    // look crap with nearest neighbour (pixel) scaling)
    const fitX = Math.floor(gl.canvas.width / this.state.virtualScreen.width);
    const fitY = Math.floor(gl.canvas.height / this.state.virtualScreen.height);
    const scale = Math.max(1.0 * devicePixelRatio, Math.min(fitX, fitY));

    this.state.virtualScreenProgram.render(
      this.state.virtualScreen.attachments[0],
      Vec2.scalarMult(virtualScreenSize, scale)
    );
  }
}
