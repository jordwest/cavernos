precision mediump float;

varying vec2 v_texCoord;

uniform sampler2D virtualScreen;
uniform vec2 virtualScreenSize;
uniform sampler2D scanlineTexture;

void main() {
  vec4 col = texture2D(virtualScreen, v_texCoord);

  // Grab a pixel to the left for some bleed
  vec2 left = vec2(-1.0 / virtualScreenSize.x, 0.0);
  vec4 bleedCol = texture2D(virtualScreen, v_texCoord + left);
  vec4 bleedCol2 = texture2D(virtualScreen, v_texCoord + left + left);

  vec4 scanline = texture2D(scanlineTexture, v_texCoord * virtualScreenSize);

  gl_FragColor = (col * 0.6 + bleedCol * 0.2 + bleedCol2 * 0.2) * scanline;
}
