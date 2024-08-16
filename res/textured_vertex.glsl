#version 140

in vec2 position;
in vec2 tex_coords;

out vec2 frag_pos;
out vec2 frag_tex_coords;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    frag_pos = position;
}
