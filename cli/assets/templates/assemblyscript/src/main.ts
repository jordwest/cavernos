import { COLS, ROWS, CHARS, FG_COLOR } from './cavernos';

// Reserve and export some memory for OS operations
export const OS = new ArrayBuffer(262144);

export function init(os_pointer: usize): usize {
  store<u8>(os_pointer + COLS, 80);
  store<u8>(os_pointer + ROWS, 24);
  
  return 0;
}

export function frame (os_pointer: usize, app_pointer: usize, dt: f64): void {
  store<u8>(os_pointer + CHARS, 1);
  store<u8>(os_pointer + CHARS + 1, 1);

  store<u8>(os_pointer + FG_COLOR, 1);
  store<u8>(os_pointer + FG_COLOR + 1, 1);
}