import * as twgl from "twgl.js";
import vertSrc from "./virtualscreen.vert";
import fragSrc from "./virtualscreen.frag";
import { quad } from "../../util/quad";
import { Vec2 } from "../../util/vec2";

type State = {
  gl: WebGLRenderingContext;

  programInfo: twgl.ProgramInfo;
  bufferInfo: twgl.BufferInfo;
  scanlineTexture: WebGLTexture;
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
    const scanlineTexture = twgl.createTexture(gl, {
      src: new Uint8Array([
        // A
        255,
        255,
        255,
        255,

        //
        255,
        255,
        255,
        255,

        //
        255,
        255,
        255,
        255,

        //
        90,
        90,
        90,
        255,
      ]),
      width: 1,
      height: 4,
    });

    this.state = {
      gl,
      programInfo,
      bufferInfo,
      scanlineTexture,
    };
  }

  render(virtualScreen: WebGLTexture, drawSize: Vec2, virtualScreenSize: Vec2) {
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
      virtualScreenSize: [virtualScreenSize.x, virtualScreenSize.y],
      scale,
      scanlineTexture: this.state.scanlineTexture,
    });
    twgl.drawBufferInfo(gl, this.state.bufferInfo);
  }
}
