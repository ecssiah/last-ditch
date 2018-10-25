#version 330 core
layout (location = 0) in vec2 aVertexPos;
layout (location = 1) in vec2 aWorldPos;
layout (location = 2) in vec2 aTexCoord;

out vec2 tex_coord;

uniform mat4 view;
uniform mat4 projection;

void main()
{
  mat4 model = mat4(1.0);
  model[3][0] = aWorldPos[0];
  model[3][1] = aWorldPos[1];

  gl_Position = projection * view * model * vec4(aVertexPos, 1.0, 1.0);
  tex_coord = aTexCoord;
}
