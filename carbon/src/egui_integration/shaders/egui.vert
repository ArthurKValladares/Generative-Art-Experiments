#version 450

layout(location = 0) in vec2 i_pos;
layout(location = 1) in vec2 i_uv;
layout(location = 2) in vec4 i_color;

layout (location = 0) out vec2 o_uv;
layout (location = 1) out vec4 o_color;

layout( push_constant ) uniform constants
{
	vec2 screen_size;
} PushConstants;

vec4 uint_to_color(uint u) {
    float r = ((u & 0xff000000) >> 24) / 255.0;
    float g = ((u & 0x00ff0000) >> 16) / 255.0;
    float b = ((u & 0x0000ff00) >> 8) / 255.0;
    float a = (u & 0x000000ff) / 255.0;
    return vec4(r, g, b, a);
}

vec3 srgb_to_linear(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(0.04045));
    vec3 lower = srgb / vec3(12.92);
    vec3 higher = pow((srgb + vec3(0.055)) / vec3(1.055), vec3(2.4));
    return mix(higher, lower, cutoff);
}

void main() {
    o_uv = i_uv;
    o_color = vec4(srgb_to_linear(i_color.rgb), i_color.a);
    
    gl_Position = vec4(
        2.0 * i_pos.x / PushConstants.screen_size.x - 1.0,
        1.0 - 2.0 * i_pos.y / PushConstants.screen_size.y,
        0.0,
        1.0
    );
}