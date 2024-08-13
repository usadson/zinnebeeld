#version 140

in vec2 position;
out vec2 frag_pos;

uniform mat4 matrix;

void main() {
    gl_Position = matrix * vec4(position, 0.0, 1.0);
    frag_pos = position;
}
