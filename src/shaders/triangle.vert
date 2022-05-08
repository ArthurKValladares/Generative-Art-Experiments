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

layout (location = 0) out vec4 o_color;

void main() {
    Vertex v=vertices[gl_VertexIndex];

    o_color = vec4(v.uv, 0.0, 1.0);
    gl_Position = v.pos;
}