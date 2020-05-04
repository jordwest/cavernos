
precision mediump float;

varying vec2 v_texCoord;

void main() {
  // spriteOffset = (v_texCoord.x * cols) % 1
  gl_FragColor = vec4(v_texCoord.x, 0.0, v_texCoord.y, 1.0);
}
