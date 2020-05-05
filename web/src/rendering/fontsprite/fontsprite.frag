
precision mediump float;

varying vec2 v_texCoord;

// Colors used for rendering
uniform sampler2D narrowFont;
uniform sampler2D squareFont;
uniform sampler2D palette;

// Tables used for lookups
uniform sampler2D charsTexture;
uniform sampler2D backgroundColorTable;

uniform vec2 fontTileCount;
uniform vec2 screenTileCount;

void main() {
  vec4 charSample = texture2D(charsTexture, v_texCoord);
  vec2 char = (charSample.xy * 255.0);

  vec2 bgColLoc = texture2D(backgroundColorTable, v_texCoord).xy * 255.0;
  vec4 backgroundColor = texture2D(palette, (bgColLoc + 0.5) / 16.0);

  bool doubleWidth = charSample.z > 0.5;

  vec2 fontTileCountScaled = doubleWidth
    ? vec2(fontTileCount.x * 2.0, fontTileCount.y)
    : fontTileCount;
  vec2 spriteSize = 1.0 / fontTileCountScaled;
  vec2 fontTextureStartOffset = char * spriteSize;

  vec2 spriteOffset = vec2(
    mod((v_texCoord.x * screenTileCount.x), 1.0),
    mod((v_texCoord.y * screenTileCount.y), 1.0)
  ) * spriteSize;

  vec2 fontTextureOffset = fontTextureStartOffset + spriteOffset;

  vec4 fontCol = doubleWidth
    ? texture2D(squareFont, fontTextureOffset)
    //? vec4(spriteOffset / spriteSize, 0.0, 1.0)
    : texture2D(narrowFont, fontTextureOffset);

  //gl_FragColor = vec4(v_texCoord.x, 0.0, v_texCoord.y, 1.0);
  gl_FragColor = vec4(mix(backgroundColor.rgb, fontCol.rgb, fontCol.a), 1.0);
}
