precision mediump float;

attribute vec2 position;
attribute vec2 texCoord;

varying vec2 v_texCoord;

void main() {
  v_texCoord = texCoord;
  gl_Position = vec4(position, 0.0, 1.0);
}
