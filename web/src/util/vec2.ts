export type Vec2 = {
  x: number;
  y: number;
};

export namespace Vec2 {
  export function scalarMult(v: Vec2, scalar: number) {
    return {
      x: v.x * scalar,
      y: v.y * scalar,
    };
  }
}
