#version 330 core

in vec3 point_colour;
in float point_opacity;
//in vec3 point_normals;
//in vec3 fragment_position;

out vec4 fragment_colour;

//uniform float ambient_strength;
//uniform vec3 ambient_colour;
//uniform vec3 light_source_pos;
//uniform vec3 light_source_colour;

void main() {
    fragment_colour = vec4(point_colour, point_opacity);
}