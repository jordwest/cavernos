import * as twgl from "twgl.js";

export class LookupTable {
  gl: WebGLRenderingContext;
  texture: WebGLTexture;
  backingData: Uint8Array;

  constructor(gl: WebGLRenderingContext) {
    this.gl = gl;
    this.backingData = new Uint8Array(4);
    const opts = {
      src: this.backingData,
      width: 1,
      height: 1,
      format: gl.RGBA,
      min: gl.NEAREST,
      mag: gl.NEAREST,
      wrap: gl.CLAMP_TO_EDGE,
    };
    this.texture = twgl.createTexture(gl, opts);
  }

  // TODO
  updateWithColorData(cols: number, rows: number, data: DataView) {}

  updateWithCharData(cols: number, rows: number, data: DataView) {
    const { gl, texture } = this;

    let offset = 0;
    const chars = cols * rows;
    if (this.backingData.length < chars * 4) {
      this.backingData = new Uint8Array(chars * 4);
    }

    const setChar = (i: number, x: number, y: number, doubleWidth: boolean) => {
      this.backingData[i * 4] = x;
      this.backingData[i * 4 + 1] = y;
      // Flag to tell the shader to double number of tiles
      this.backingData[i * 4 + 2] = doubleWidth ? 255 : 0;
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

    // Finally, upload the texture to the GPU
    twgl.setTextureFromArray(gl, texture, this.backingData, {
      min: gl.NEAREST,
      mag: gl.NEAREST,
      width: cols,
      height: rows,
      wrap: gl.CLAMP_TO_EDGE,
    });
  }
}
