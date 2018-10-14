#ifndef GL_LOAD_SHADER_H
#define GL_LOAD_SHADER_H

#include <fstream>
#include <iostream>

#include <GL/glew.h>

const std::string ImportShaderSource(const std::string& filename)
{
  std::string content;
  std::ifstream fs(filename);

  if (!fs.is_open())
  {
    std::cerr << "Could not read file " << filename << std::endl;
    return "";
  }

  std::string line;
  while (!fs.eof())
  {
    std::getline(fs, line);
    content.append(line + "\n");
  }

  fs.close();
  return content;
}

unsigned int GLLoadShader(
  const std::string& vert_shader_path, const std::string& frag_shader_path
) {
  int success;
  char info_log[512];
  unsigned int shader_prog; 

  unsigned int vert_shader = glCreateShader(GL_VERTEX_SHADER);

  const std::string vert_shader_str = ImportShaderSource(vert_shader_path);
  const GLchar* vert_shader_src = vert_shader_str.c_str(); 

  glShaderSource(vert_shader, 1, &vert_shader_src, nullptr);
  glCompileShader(vert_shader);

  glGetShaderiv(vert_shader, GL_COMPILE_STATUS, &success);

  if (!success)
  {
    glGetShaderInfoLog(vert_shader, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }

  unsigned int frag_shader = glCreateShader(GL_FRAGMENT_SHADER);

  const std::string frag_shader_str = ImportShaderSource(frag_shader_path);
  const GLchar* frag_shader_src = frag_shader_str.c_str();

  glShaderSource(frag_shader, 1, &frag_shader_src, nullptr);
  glCompileShader(frag_shader);

  glGetShaderiv(frag_shader, GL_COMPILE_STATUS, &success);

  if (!success)
  {
    glGetShaderInfoLog(frag_shader, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }

  shader_prog = glCreateProgram();

  glAttachShader(shader_prog, vert_shader);
  glAttachShader(shader_prog, frag_shader);

  glLinkProgram(shader_prog);

  glGetProgramiv(shader_prog, GL_LINK_STATUS, &success);

  if (!success)
  {
    glGetProgramInfoLog(shader_prog, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }

  glDeleteShader(vert_shader);
  glDeleteShader(frag_shader);

  return shader_prog;
} 

#endif // GL_LOAD_SHADER_H
