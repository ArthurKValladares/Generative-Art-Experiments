#version 450

layout (binding = 2) uniform sampler2D samplerColor;

layout (location = 0) in vec2 i_uv;
layout (location = 1) in vec4 i_color;

layout (location = 0) out vec4 uFragColor;

void main() {
    uFragColor = texture(samplerColor, i_uv) * i_color;
}