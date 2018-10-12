#version 330 core
in vec3 vertex_color;
in vec2 tex_coord;

out vec4 FragColor;

uniform sampler2D test_texture;

void main()
{
  FragColor = texture(test_texture, tex_coord);
}
