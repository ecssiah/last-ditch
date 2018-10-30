#version 330 core
layout (location = 0) in vec2 aVertexPos;
layout (location = 1) in vec3 aWorldPos;
layout (location = 2) in vec4 aTexCoord;

out vec2 tex_coord;
out float tileset_id;

uniform mat4 view;
uniform mat4 projection;

void main()
{
  mat4 model = mat4(1.0);
  model[3].xy = aWorldPos.xy;

  // LBRT => 0123
  // 0 => LB, 1 => LT, 2 => RB, 3 => RT

  if (gl_VertexID == 0) {
    tex_coord.x = aTexCoord[0];
    tex_coord.y = aTexCoord[1];
  } else if (gl_VertexID == 1) {
    tex_coord.x = aTexCoord[0];
    tex_coord.y = aTexCoord[3];
  } else if (gl_VertexID == 2) {
    tex_coord.x = aTexCoord[2];
    tex_coord.y = aTexCoord[1];
  } else if (gl_VertexID == 3) {
    tex_coord.x = aTexCoord[2];
    tex_coord.y = aTexCoord[3];
  }

  tileset_id = aWorldPos.z;
  gl_Position = projection * view * model * vec4(aVertexPos, 0.0, 1.0);
}
