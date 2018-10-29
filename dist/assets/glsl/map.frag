#version 330 core
in vec2 tex_coord;
in float tileset_id;

out vec4 FragColor;

uniform sampler2D character_tileset;
uniform sampler2D map_tileset;
uniform sampler2D object_tileset;

void main()
{
  if (tileset_id == 0.0) {
    FragColor = texture(character_tileset, tex_coord);
  } else if (tileset_id == -1.0) {
    FragColor = texture(object_tileset, tex_coord);
  } else if (tileset_id == -2.0) {
    FragColor = texture(map_tileset, tex_coord);
  }
}
