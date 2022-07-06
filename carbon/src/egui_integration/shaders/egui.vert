#version 450

struct Vertex
{
	vec2 pos;
    vec2 uv;
    uint color;
};

layout(set = 0, binding = 0) readonly buffer Vertices
{
	Vertex vertices[];
};

layout (location = 0) out vec2 o_uv;
layout (location = 1) out vec4 o_color;

vec4 color_from_u32(uint color) {
    uint red = (color & 0xff000000) >> 24;
    uint green = (color & 0x00ff0000) >> 16;
    uint blue = (color & 0x0000ff00) >> 8;
    uint alpha = (color & 0x000000ff);

    return vec4(red / 255.0, green / 255.0, blue / 255.0, alpha / 255.0);
}

void main() {
    Vertex v=vertices[gl_VertexIndex];

    o_uv = v.uv;
    o_color = color_from_u32(v.color);
    gl_Position = vec4(v.pos, 0.0, 1.0);
}