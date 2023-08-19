#version 100

precision mediump float;

attribute vec2 vertex;
attribute vec2 tex_coord;

uniform mat4 mvp;
uniform vec2 pos, size;
uniform float shear;

varying vec2 fragment_tex_coord;

void main() {
  vec2 v = vertex * size;
  vec2 pos = pos + vec2(v.x + shear * (size.y - v.y), v.y);
  gl_Position = mvp * vec4(pos, 0.0, 1.0);
  fragment_tex_coord = tex_coord;
}