#version 300 es

precision highp float;

in vec2 a_pos;
// in vec4 a_color;
in vec2 a_tex_coord;

uniform vec2 u_resolution;
// uniform mat4 u_transformation;

// out vec4 v_color;
out vec2 v_tex_coord;

void main() {
  vec2 zero_to_one = a_pos / u_resolution;
  vec2 zero_to_two = zero_to_one * 2.0;
  vec2 clip_space = zero_to_two - 1.0;

  // v_color = a_color;
  v_tex_coord = a_tex_coord;
  gl_Position = vec4(clip_space * vec2(1, -1), 0, 1);
  // gl_Position = vec4(clip_space * vec2(1, -1), 0, 1) * u_transformation;
  // gl_Position = vec4(a_pos, 0, 1);
}
