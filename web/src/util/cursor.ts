export class Float32Cursor {
  data: Float32Array;
  offset: number;

  constructor(length: number) {
    this.data = new Float32Array(length);
    this.offset = 0;
  }

  push(a: number) {
    this.data[this.offset] = a;
    this.offset += 1;
  }

  push2(a: number, b: number) {
    this.push(a);
    this.push(b);
  }
}

export class ArrayBufferCursor {
  view: DataView;
  offset: number;

  constructor(buffer: ArrayBuffer) {
    this.view = new DataView(buffer);
    this.offset = 0;
  }

  f2(f32_a: number, f32_b: number) {
    this.view.setFloat32(this.offset, f32_a);
    this.offset += 4;
    this.view.setFloat32(this.offset, f32_b);
    this.offset += 4;
  }
}
