#version 100

precision mediump float;

attribute vec2 vertex;
attribute vec2 tex_coord;

uniform mat4 mvp;

varying vec2 fragment_tex_coord;

void main() {
  gl_Position = mvp * vec4(vertex, 0.0, 1.0);
  fragment_tex_coord = tex_coord;
}