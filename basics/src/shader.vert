#version 300 es

precision highp float;

// uniform mat4 u_Transform;

in vec4 position;
in vec4 idkColor;
uniform vec4 u_offset;
uniform mat4 u_matrix;
uniform mat4 u_matrix2;

out vec4 inColor;

void main() {
    inColor = idkColor;
    gl_Position = u_matrix * u_matrix2 * (position + u_offset);
    // gl_Position = u_offset + position;
}
