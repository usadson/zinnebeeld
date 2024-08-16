#version 140

out vec4 out_color;

in vec2 frag_pos;
in vec2 frag_tex_coords;

uniform sampler2D tex;

void main() {
    // float dist = 1 - length(frag_pos);

    // if (dist < 0.333) {
    //     discard;
    // }

    // out_color = vec4(frag_tex_coords, 1.0, 1.0);// texture(tex, frag_tex_coords);
    out_color = vec4(1, 1, 1, 1);
}
