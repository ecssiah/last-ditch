#version 330 core
layout (location = 0) in vec2 aVertexPos;
layout (location = 1) in vec2 aTexOffset;
layout (location = 2) in vec3 aWorldPos;
layout (location = 3) in vec4 aTexCoord;

out vec2 tex_coord;
out float tileset_id;

uniform mat4 view;
uniform mat4 projection;

void main()
{
  mat4 model = mat4(1.0);
  model[3].xy = aWorldPos.xy;

  tex_coord = vec2(aTexCoord[0] + aTexOffset[0], aTexCoord[1] + aTexOffset[1]);

  tileset_id = aWorldPos.z;
  gl_Position = projection * view * model * vec4(aVertexPos, 0.0, 1.0);
}
