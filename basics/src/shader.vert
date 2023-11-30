#version 300 es

precision highp float;

// uniform mat4 u_Transform;

in vec4 position;
in vec4 idkColor;
uniform mat4 projection;
uniform mat4 translation;
uniform mat4 rotationX;
uniform mat4 rotationY;
uniform mat4 rotationZ;

out vec4 inColor;

void main() {
    inColor = idkColor;
    gl_Position = position * rotationX * rotationY * rotationZ * translation * projection;
}
