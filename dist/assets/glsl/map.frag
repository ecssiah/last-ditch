#version 330 core
in vec2 tex_coord;

out vec4 FragColor;

uniform sampler2D character_tileset;
uniform sampler2D map_tileset;
uniform sampler2D object_tileset;

void main()
{
  FragColor = texture(character_tileset, tex_coord);
}
