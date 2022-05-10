#version 450

struct Vertex
{
	vec4 pos;
    vec2 uv;
    vec2 pad;
};

layout(set = 0, binding = 0) readonly buffer Vertices
{
	Vertex vertices[];
};

layout (location = 0) out vec2 o_uv;

void main() {
    Vertex v=vertices[gl_VertexIndex];

    o_uv = v.uv;
    gl_Position = v.pos;
}