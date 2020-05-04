import * as twgl from "twgl.js";
import vertSrc from "./virtualscreen.vert";
import fragSrc from "./virtualscreen.frag";
import { quad } from "../../util/quad";

type State = {
  gl: WebGLRenderingContext;

  programInfo: twgl.ProgramInfo;
  bufferInfo: twgl.BufferInfo;
};

export class VirtualScreenProgram {
  state: State;
  constructor(gl: WebGLRenderingContext) {
    const arrays = {
      position: {
        numComponents: 2,
        data: quad(-1, -1, 1, 1),
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

  render(virtualScreen: WebGLTexture, drawSize: Vec2) {
    const { gl } = this.state;

    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    gl.enable(gl.BLEND);

    const scale = [drawSize.x / gl.canvas.width, drawSize.y / gl.canvas.height];

    gl.useProgram(this.state.programInfo.program);
    twgl.setBuffersAndAttributes(
      gl,
      this.state.programInfo,
      this.state.bufferInfo
    );
    twgl.setUniforms(this.state.programInfo, {
      virtualScreen,
      scale,
    });
    twgl.drawBufferInfo(gl, this.state.bufferInfo);
  }
}
