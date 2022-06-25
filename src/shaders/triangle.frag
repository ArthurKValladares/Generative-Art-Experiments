#version 450

// Setting it to 50 for now, figure out how to do this properly later.
// Do I use specialization constants, or just set to a max value?
layout (binding = 2) uniform sampler2D samplerColor[50];

layout (location = 0) in vec2 i_uv;
layout (location = 1) in vec4 i_color;

layout (location = 0) out vec4 uFragColor;

//push constants block
layout( push_constant ) uniform constants
{
	uint texture_index;
    uint pad_1;
    uint pad_2;
    uint pad_3;
} PushConstants;


void main() {
    uFragColor = texture(samplerColor[PushConstants.texture_index], i_uv) * i_color;
}