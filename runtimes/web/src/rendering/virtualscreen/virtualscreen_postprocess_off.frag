precision mediump float;

varying vec2 v_texCoord;

uniform sampler2D virtualScreen;

void main() {
  vec4 col = texture2D(virtualScreen, v_texCoord);

  gl_FragColor = col;
}
