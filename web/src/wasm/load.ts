interface WasmExports {
  frame(): void;
  memory: {
    buffer: ArrayBuffer;
  };
  CONFIG: { value: number };
  SCREEN: { value: number };
}

type State = {
  instance: WebAssembly.Instance;
  exports: WasmExports;
};

export class WasmProgram {
  state: State;

  constructor(state: State) {
    this.state = state;
  }

  static async load(wasmSrc: string) {
    const { instance } = await WebAssembly.instantiateStreaming(
      fetch(wasmSrc),
      {
        env: {
          prn: (addr: number, len: number) => {
            const exports = (instance.exports as unknown) as WasmExports;
            const raw = new Uint8ClampedArray(exports.memory.buffer, addr, len);
            const decoder = new TextDecoder("utf8");
            console.info(decoder.decode(raw));
          },
        },
      }
    );

    const exports = (instance.exports as unknown) as WasmExports;
    return new WasmProgram({ instance, exports });
  }

  tick() {
    this.state.exports.frame();
  }

  get config() {
    const view = new DataView(
      this.state.exports.memory.buffer,
      this.state.exports.CONFIG.value,
      2
    );
    return {
      cols: view.getUint8(0),
      rows: view.getUint8(1),
    };
  }

  get screen() {
    const { rows, cols } = this.config;
    return new DataView(
      this.state.exports.memory.buffer,
      this.state.exports.SCREEN.value,
      rows * cols
    );
  }
}
