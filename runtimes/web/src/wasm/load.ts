// For some reason iOS Safari doesn't return an object, but just the address directly
import {CONFIG_LAYOUT} from "../memory-layout";

type ExportAddress = number | { value: number };
const getExportAddress = (v: ExportAddress): Pointer =>
  (typeof v === "number" ? v : v.value) as Pointer;

type Pointer = number & { __pointer: never };
type Offset = number & { __offset: never };

interface WasmExports {
  init(osAddr: Pointer): Pointer;
  frame(osAddr: Pointer, appStateAddr: Pointer, dt: number): void;
  memory: {
    buffer: ArrayBuffer;
  };
  OS: ExportAddress;
}

type State = {
  instance: WebAssembly.Instance;
  exports: WasmExports;
};

const configOffset = 0 as Offset;
const inputsOffset = (configOffset + 256) as Offset;
const reservedOffset = (inputsOffset + 256) as Offset;
const screenOffset = (reservedOffset + 2560) as Offset;
const bgColorOffset = (screenOffset + 65536) as Offset;
const fgColorOffset = (bgColorOffset + 65536) as Offset;

export class WasmProgram {
  state: State;
  appStateAddr: Pointer;

  constructor(state: State) {
    this.state = state;
    this.appStateAddr = this.state.exports.init(this.osAddr);
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

  get osAddr(): Pointer {
    return getExportAddress(this.state.exports.OS);
  }

  tick(dt: number) {
    this.state.exports.frame(this.osAddr, this.appStateAddr, dt);
  }

  setInput(address: number, value: number) {
    const inputsAddr = getExportAddress(this.state.exports.OS) + inputsOffset;
    const view = new DataView(this.state.exports.memory.buffer, inputsAddr);
    view.setUint8(address, value);
  }

  get config() {
    const view = new DataView(
      this.state.exports.memory.buffer,
      getExportAddress(this.state.exports.OS) + configOffset,
      4
    );
    
    return new ConfigData(view);
  }

  get screen() {
    return new ScreenData(
      this.state.exports.OS,
      screenOffset,
      this.state.exports.memory.buffer,
      this.config.cols,
      this.config.rows
    );
  }

  get bgColors() {
    return new ScreenData(
      this.state.exports.OS,
      bgColorOffset,
      this.state.exports.memory.buffer,
      this.config.cols,
      this.config.rows
    );
  }

  get fgColors() {
    return new ScreenData(
      this.state.exports.OS,
      fgColorOffset,
      this.state.exports.memory.buffer,
      this.config.cols,
      this.config.rows
    );
  }
}

export class ConfigData {
  private data: DataView;
  
  constructor(view: DataView) {
    this.data = view;
  }
  
  get rows() {
    return this.data.getUint8(CONFIG_LAYOUT.ROWS);
  }
  get cols() {
    return this.data.getUint8(CONFIG_LAYOUT.COLUMNS);
  }

  set max_rows(val: number) {
    this.data.setUint8(CONFIG_LAYOUT.MAX_ROWS, val);
  }
  set max_cols(val: number) {
    this.data.setUint8(CONFIG_LAYOUT.MAX_COLUMNS, val);
  }
}

export class ScreenData {
  private updateView: DataView;
  cells: DataView;

  constructor(
    osAddr: ExportAddress,
    offset: Offset,
    buffer: ArrayBuffer,
    cols: number,
    rows: number
  ) {
    const addr = getExportAddress(osAddr) + offset;

    this.updateView = new DataView(buffer, addr, 1);
    this.cells = new DataView(buffer, addr + 1, rows * cols);
  }

  get update(): boolean {
    return this.updateView.getUint8(0) > 0;
  }

  set update(v: boolean) {
    this.updateView.setUint8(0, v ? 1 : 0);
  }
}
