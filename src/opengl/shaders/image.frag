#version 100

precision mediump float;

uniform sampler2D texture0;

varying vec2 fragment_tex_coord;

void main() {
  vec4 tex_data = texture(texture0, fragment_tex_coord);
  gl_FragColor = tex_data;
}