import * as twgl from "twgl.js";
import vertSrc from "./fontsprite.vert";
import fragSrc from "./fontsprite.frag";
import { Float32Cursor } from "../../util/cursor";
import { quad } from "../../util/quad";

type State = {
  gl: WebGLRenderingContext;

  programInfo: twgl.ProgramInfo;
  bufferInfo: twgl.BufferInfo;
  fontTexture: WebGLTexture;
};

export class FontSpriteProgram {
  state: State;
  constructor(gl: WebGLRenderingContext, fontTextureSrc: string) {
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
    const fontTexture = twgl.createTexture(gl, { src: fontTextureSrc });

    this.state = {
      gl,
      programInfo,
      bufferInfo,
      fontTexture,
    };
  }

  render() {
    const { gl } = this.state;

    gl.useProgram(this.state.programInfo.program);
    twgl.setBuffersAndAttributes(
      gl,
      this.state.programInfo,
      this.state.bufferInfo
    );
    twgl.drawBufferInfo(gl, this.state.bufferInfo);
  }
}
