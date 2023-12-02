#version 300 es

precision highp float;

// uniform vec4 u_color;
uniform sampler2D u_texture;

// in vec4 v_color;
in vec2 v_tex_coord;

out vec4 out_color;

void main() {
  // out_color = v_color;
  out_color = texture(u_texture, v_tex_coord);
  out_color = vec4(out_color.yxxw);
}
