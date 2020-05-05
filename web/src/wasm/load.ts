// For some reason iOS Safari doesn't return an object, but just the address directly
type ExportAddress = number | { value: number };
const getExportAddress = (v: ExportAddress) =>
  typeof v === "number" ? v : v.value;

interface WasmExports {
  frame(): void;
  memory: {
    buffer: ArrayBuffer;
  };
  CONFIG: ExportAddress;
  SCREEN: ExportAddress;
  INPUTS: ExportAddress;
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
    const imports = {
      env: {
        prn: (addr: number, len: number) => {
          const exports = (instance.exports as unknown) as WasmExports;
          const raw = new Uint8ClampedArray(exports.memory.buffer, addr, len);
          const decoder = new TextDecoder("utf8");
          console.info(decoder.decode(raw));
        },
      },
    };

    let instance: WebAssembly.Instance;
    if (WebAssembly.instantiateStreaming != null) {
      const m = await WebAssembly.instantiateStreaming(fetch(wasmSrc), imports);
      instance = m.instance;
    } else {
      const response = await fetch(wasmSrc);
      const m = await WebAssembly.instantiate(
        await response.arrayBuffer(),
        imports
      );
      instance = m.instance;
    }

    const exports = (instance.exports as unknown) as WasmExports;
    console.log("exports", Object.keys(exports));
    return new WasmProgram({ instance, exports });
  }

  tick() {
    this.state.exports.frame();
  }

  setInput(address: number, value: number) {
    const inputsAddr = getExportAddress(this.state.exports.INPUTS);
    const view = new DataView(this.state.exports.memory.buffer, inputsAddr);
    view.setUint8(address, value);
  }

  get config() {
    const view = new DataView(
      this.state.exports.memory.buffer,
      getExportAddress(this.state.exports.CONFIG),
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
      getExportAddress(this.state.exports.SCREEN),
      rows * cols
    );
  }
}
