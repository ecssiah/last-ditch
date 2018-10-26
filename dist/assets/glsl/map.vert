#version 330 core
layout (location = 0) in vec2 aVertexPos;
layout (location = 1) in vec2 aWorldPos;
layout (location = 2) in vec4 aTexCoord;

out vec2 tex_coord;

uniform mat4 view;
uniform mat4 projection;

void main()
{
  mat4 model = mat4(1.0);
  model[3].xy = aWorldPos;

  if (gl_VertexID == 0 || gl_VertexID == 1 || gl_VertexID == 4) {
    tex_coord.x = aTexCoord[2];
  } else {
    tex_coord.x = aTexCoord[0];
  }

  if (gl_VertexID == 0 || gl_VertexID == 2) {
    tex_coord.y = aTexCoord[3];
  } else {
    tex_coord.y = aTexCoord[1];
  }

  gl_Position = projection * view * model * vec4(aVertexPos, 0.0, 1.0);
}
