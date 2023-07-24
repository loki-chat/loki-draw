#version 100

precision mediump float;

attribute vec2 vertex;

uniform vec2 pos, size;
uniform mat4 mvp;

varying vec2 frag_tex_coord;

void main() {
  gl_Position = mvp * vec4(pos + size * vertex, 0.0, 1.0);
  frag_tex_coord = vertex;
}