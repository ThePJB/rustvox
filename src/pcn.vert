layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_colour;
layout (location = 2) in vec3 in_normal;

uniform mat4 projection;
uniform mat4 view;

out vec4 vert_colour;

const vec3 sun_dir = normalize(vec3(0.2, 0.3, 0.4));

void main() {
    float facing_ratio = dot(in_normal, sun_dir) / 2.0 + 0.5;
    // float facing_ratio = 0.8;

    float brightness = facing_ratio * 0.5 + 0.5;

    vert_colour = vec4(in_colour.xyz * brightness, in_colour.w);
    // vert_colour = (vec4(in_pos, 1.0)).xyz;
    gl_Position = projection * view * vec4(in_pos, 1.0);
}