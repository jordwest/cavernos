import { Float32Cursor } from "./cursor";

export function quad(
  x1: number,
  y1: number,
  x2: number,
  y2: number
): Float32Array {
  const cursor = new Float32Cursor(12);
  // Bottom left
  cursor.push2(x1, y1);
  cursor.push2(x2, y2);
  cursor.push2(x1, y2);

  // Top right
  cursor.push2(x1, y1);
  cursor.push2(x2, y1);
  cursor.push2(x2, y2);

  return cursor.data;
}
