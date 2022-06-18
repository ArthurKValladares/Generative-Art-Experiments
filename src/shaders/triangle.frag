#version 450

// Setting it to 10 for now, figure out how to do this properly later.
// Do I use specialization constants, or just set to a max value?
layout (binding = 2) uniform sampler2D samplerColor[10];

layout (location = 0) in vec2 i_uv;
layout (location = 1) in vec4 i_color;

layout (location = 0) out vec4 uFragColor;

void main() {
    uFragColor = texture(samplerColor[0], i_uv) * i_color;
}