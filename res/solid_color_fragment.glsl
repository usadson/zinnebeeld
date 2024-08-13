#version 140

out vec4 out_color;

in vec2 frag_pos;

uniform vec4 color;

void main() {
    float dist = 1 - length(frag_pos);

    if (dist < 0.333) {
        discard;
    }

    out_color = color;
}
