#version 450

struct Vertex
{
	vec4 pos;
    vec4 color;
    vec2 uv;
    vec2 pad;
};

layout(set = 0, binding = 0) readonly buffer Vertices
{
	Vertex vertices[];
};

layout (location = 0) out vec2 o_uv;
layout (location = 1) out vec4 o_color;

layout( push_constant ) uniform constants
{
	uint screen_width;
    uint screen_height;
    uint pad_2;
    uint pad_3;
} PushConstants;

vec4 color_from_u32(uint color) {
    uint red = (color & 0xff000000) >> 24;
    uint green = (color & 0x00ff0000) >> 16;
    uint blue = (color & 0x0000ff00) >> 8;
    uint alpha = (color & 0x000000ff);

    return vec4(red / 255.0, green / 255.0, blue / 255.0, alpha / 255.0);
}

// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));
}

vec4 linear_from_srgba(vec4 srgba) {
   return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
    Vertex v = vertices[gl_VertexIndex];

    o_uv = v.uv;
    o_color = linear_from_srgba(v.color);
    gl_Position = vec4(
        2.0 * v.pos.x / PushConstants.screen_width - 1.0,
        1.0 - 2.0 * v.pos.y / PushConstants.screen_height,
        0.0,
        1.0
    );
}