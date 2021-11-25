layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec3 in_colour;

uniform mat4 projection;
uniform mat4 view;

out vec3 vert_colour;

void main() {
    vert_colour = in_colour;
    // vert_colour = (vec4(in_pos, 1.0)).xyz;
    // gl_Position = projection * view * 
    gl_Position = vec4(in_pos, 1.0);
}