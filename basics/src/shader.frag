#version 300 es

precision highp float;

in vec4 inColor;
out vec4 outColor;

void main() {
    // outColor = vec4(0.5, 0.2, 1, 1);
    outColor = inColor;
}
