#version 100

precision mediump float;

uniform vec4 col;
uniform float smoothness;
uniform float inner_rad;

varying vec2 frag_tex_coord;

float map(float value, float inMin, float inMax, float outMin, float outMax) {
  return outMin + (outMax - outMin) * (value - inMin) / (inMax - inMin);
}

void main() {
  float l = length(frag_tex_coord);
  float hs = smoothness / 2.0;
  float or = clamp(map(l, 1.0 - hs, 1.0 + hs, 1.0, 0.0), 0.0, 1.0);
  float ir = clamp(map(l, inner_rad - hs, inner_rad + hs, 1.0, 0.0), 0.0, 1.0);
  gl_FragColor = vec4(col.rgb, col.a * (or -ir));
}