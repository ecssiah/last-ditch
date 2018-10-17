#version 330 core
in vec3 vertex_color;
in vec2 tex_coord;

out vec4 FragColor;

uniform sampler2D object_tileset;
uniform sampler2D character_tileset;

void main()
{
  FragColor = texture(object_tileset, tex_coord);
}
