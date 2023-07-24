#version 100

precision mediump float;

uniform vec4 col;

void main() {
  gl_FragColor = col;
}