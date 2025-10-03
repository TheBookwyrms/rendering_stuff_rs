#version 330 core

in vec3 point_colour;
in float point_opacity;

out vec4 fragment_colour;

uniform float ambient_strength;


void main() {
    fragment_colour = vec4(point_colour, ambient_strength);
}