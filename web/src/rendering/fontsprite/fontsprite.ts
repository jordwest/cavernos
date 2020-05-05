import * as twgl from "twgl.js";
import vertSrc from "./fontsprite.vert";
import fragSrc from "./fontsprite.frag";
import { quad } from "../../util/quad";

type State = {
  gl: WebGLRenderingContext;

  programInfo: twgl.ProgramInfo;
  bufferInfo: twgl.BufferInfo;
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

    this.state = {
      gl,
      programInfo,
      bufferInfo,
    };
  }

  render(opts: {
    rows: number;
    cols: number;
    narrowFont: WebGLTexture;
    squareFont: WebGLTexture;
    palette: WebGLTexture;
    charsTable: WebGLTexture;
    backgroundColorTable: WebGLTexture;
  }) {
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
      narrowFont: opts.narrowFont,
      squareFont: opts.squareFont,
      charsTexture: opts.charsTable,
      palette: opts.palette,
      backgroundColorTable: opts.backgroundColorTable,
      fontTileCount: [16, 16],
      screenTileCount: [opts.cols, opts.rows],
    });
    twgl.drawBufferInfo(gl, this.state.bufferInfo);
  }
}
