#version 450

struct Vertex
{
	vec4 pos;
    vec4 color;
    vec4 normal;
    vec2 uv;
    vec2 pad;
};

layout(set = 0, binding = 0) readonly buffer Vertices
{
	Vertex vertices[];
};

layout(set = 0, binding = 2) uniform UniformBufferObject
{
	mat4 proj;
} ubo;

layout (location = 0) out vec2 o_uv;
layout (location = 1) out vec4 o_color;

//push constants block
layout( push_constant ) uniform constants
{
	mat4 model_matrix;
} PushConstants;

void main() {
    Vertex v=vertices[gl_VertexIndex];

    o_uv = v.uv;
    o_color = (vec4(v.normal.rgb, 1.0) * 0.7) +  v.color;
    gl_Position = PushConstants.model_matrix * ubo.proj * vec4(v.pos.xyz, 1.0);
}