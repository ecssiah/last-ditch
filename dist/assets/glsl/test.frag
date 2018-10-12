#version 330 core
in vec3 vertex_color;
out vec4 FragColor;

uniform vec4 my_color;

void main()
{
  FragColor = vec4(vertex_color, 1.0); 
}
