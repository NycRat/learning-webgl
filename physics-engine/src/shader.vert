#version 300 es

precision highp float;

in vec4 a_pos;
in vec3 a_normal;
// in vec4 a_color;

uniform mat4 u_transformation;

// out vec4 v_color;
out vec3 v_normal;

void main() {
  gl_Position = a_pos * u_transformation;
  v_normal = a_normal;
  //
  // float fudge = 1.0;
  // float zToDivideBy = 1.0 + gl_Position.z * fudge;
  //
  // gl_Position = vec4(gl_Position.xy / zToDivideBy, gl_Position.zw);

  // gl_Position = vec4(a_pos, 0, 1);
  // v_color = a_color;
}
