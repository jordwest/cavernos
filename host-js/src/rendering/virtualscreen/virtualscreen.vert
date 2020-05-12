precision mediump float;

attribute vec2 position;
attribute vec2 texCoord;

varying vec2 v_texCoord;

uniform vec2 scale;

void main() {
  v_texCoord = texCoord;
  gl_Position = vec4(position * scale, 0.0, 1.0);
}
