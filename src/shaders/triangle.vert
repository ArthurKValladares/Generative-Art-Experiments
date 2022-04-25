#version 450

struct Vertex
{
	vec4 pos;
    vec4 col;
};

layout(set = 0, binding = 0) readonly buffer Vertices
{
	Vertex vertices[];
};

layout (location = 0) out vec4 o_color;

void main() {
    Vertex v=vertices[gl_VertexIndex];

    o_color = v.col;
    gl_Position = v.pos;
}