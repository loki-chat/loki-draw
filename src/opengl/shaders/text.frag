#version 100

precision mediump float;

uniform vec4 col;
uniform sampler2D texture0;

varying vec2 fragment_tex_coord;

void main() {
  vec4 tex_data = texture2D(texture0, fragment_tex_coord);
  // gl_FragColor = vec4(col.rgb, col.a * tex_data.r);
  // not sure if this needs to be a max instead of an average, but it for sure can't just be the read channel (SubpixelAA)!
  gl_FragColor = vec4(col.rgb, col.a * (tex_data.r + tex_data.g + tex_data.b) / 3.0); 
}