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
    o_uv = i_uv;
    o_color = linear_from_srgba(i_color);
    
    gl_Position = vec4(
        2.0 * i_pos.x / PushConstants.screen_size.x - 1.0,
        1.0 - 2.0 * i_pos.y / PushConstants.screen_size.y,
        0.0,
        1.0
    );
}