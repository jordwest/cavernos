import * as twgl from "twgl.js";
import vertSrc from "./fontsprite.vert";
import fragSrc from "./fontsprite.frag";
import { quad } from "../../util/quad";

type State = {
  gl: WebGLRenderingContext;

  programInfo: twgl.ProgramInfo;
  bufferInfo: twgl.BufferInfo;
  charsTexture: WebGLTexture;

  cols: number;
  rows: number;
  textureData: Uint8Array;
};

export class FontSpriteProgram {
  state: State;
  constructor(gl: WebGLRenderingContext) {
    const arrays = {
      position: {
        numComponents: 2,
        data: quad(-1, 1, 1, -1),
      },
      texCoord: {
        numComponents: 2,
        data: quad(0, 0, 1, 1),
      },
    };

    const programInfo = twgl.createProgramInfo(gl, [vertSrc, fragSrc]);
    const bufferInfo = twgl.createBufferInfoFromArrays(gl, arrays);
    const textureData = new Uint8Array(4);
    const opts = {
      src: textureData,
      width: 1,
      height: 1,
      format: gl.RGBA,
      min: gl.NEAREST,
      mag: gl.NEAREST,
      wrap: gl.CLAMP_TO_EDGE,
    };
    const charsTexture = twgl.createTexture(gl, opts);

    this.state = {
      gl,
      cols: 0,
      rows: 0,
      programInfo,
      bufferInfo,
      charsTexture,
      textureData,
    };
  }

  updateScreenTexture(cols: number, rows: number, data: DataView) {
    const { gl, charsTexture } = this.state;

    let offset = 0;
    const chars = cols * rows;
    if (this.state.textureData.length < chars * 4) {
      this.state.textureData = new Uint8Array(chars * 4);
    }

    const setChar = (i: number, x: number, y: number, doubleWidth: boolean) => {
      this.state.textureData[i * 4] = x;
      this.state.textureData[i * 4 + 1] = y;
      // Flag to tell the shader to double number of tiles
      this.state.textureData[i * 4 + 2] = doubleWidth ? 255 : 0;
    };

    while (offset < chars) {
      const charCode = data.getUint8(offset);
      const x = charCode % 16;
      const y = Math.floor(charCode / 16);

      if (charCode !== 255) {
        // Narrow char
        setChar(offset, x, y, false);
      } else {
        // Double width char
        // Read the next byte to see what char to print
        const charCode = data.getUint8(offset + 1);
        const x = charCode % 16;
        const y = Math.floor(charCode / 16);

        setChar(offset, x * 2, y, true);
        setChar(offset + 1, x * 2 + 1, y, true);

        // Consume the extra char
        offset += 1;
      }

      offset += 1;
    }

    this.state.cols = cols;
    this.state.rows = rows;
    twgl.setTextureFromArray(gl, charsTexture, this.state.textureData, {
      min: gl.NEAREST,
      mag: gl.NEAREST,
      width: cols,
      height: rows,
      wrap: gl.CLAMP_TO_EDGE,
    });
  }

  render(narrowFont: WebGLTexture, squareFont: WebGLTexture) {
    const { gl } = this.state;

    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    gl.enable(gl.BLEND);

    gl.useProgram(this.state.programInfo.program);
    twgl.setBuffersAndAttributes(
      gl,
      this.state.programInfo,
      this.state.bufferInfo
    );
    twgl.setUniforms(this.state.programInfo, {
      narrowFont,
      squareFont,
      charsTexture: this.state.charsTexture,
      fontTileCount: [16, 16],
      screenTileCount: [this.state.cols, this.state.rows],
    });
    twgl.drawBufferInfo(gl, this.state.bufferInfo);
  }
}
