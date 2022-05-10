#version 450

layout (binding = 1) uniform sampler2D samplerColor;

layout (location = 0) in vec2 o_uv;
layout (location = 0) out vec4 uFragColor;

void main() {
    uFragColor = texture(samplerColor, o_uv);   
}