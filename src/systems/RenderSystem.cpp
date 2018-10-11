#include <iostream>
#include <vector>
#include <iterator>
#include <fstream>
#include <functional>

#define STB_IMAGE_IMPLEMENTATION
#include "../ext/stb/stb_image.h"

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"

namespace glfw_callback
{
  void FramebufferSizeCallback(GLFWwindow* window, int width, int height) 
  {
    glViewport(0, 0, width, height);
  }
}

RenderSystem::RenderSystem(Input& _input) :
  input(_input),
  window(nullptr)
{
}

RenderSystem::~RenderSystem()
{
  glfwTerminate();
  std::cout << "Render System Shutdown" << std::endl;
}

/////////////
// TESTING //
/////////////
void RenderSystem::CreateTestTriangle()
{
  float vertices[] = {
    -0.5f, -0.5f, 0.0f,
     0.5f, -0.5f, 0.0f,
     0.0f,  0.5f, 0.0f
  };

  unsigned int VBO;
  glGenBuffers(1, &VBO);
  glBindBuffer(GL_ARRAY_BUFFER, VBO);

  glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

  unsigned int vert_shader;
  vert_shader = glCreateShader(GL_VERTEX_SHADER);

  std::string vert_shader_str = LoadShader("assets/glsl/test.vert");
  const GLchar* vert_shader_src = vert_shader_str.c_str(); 

  glShaderSource(vert_shader, 1, &vert_shader_src, nullptr);
  glCompileShader(vert_shader);

  int success;
  char info_log[512];
  glGetShaderiv(vert_shader, GL_COMPILE_STATUS, &success);

  if (!success)
  {
    glGetShaderInfoLog(vert_shader, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }

  unsigned int frag_shader;
  frag_shader = glCreateShader(GL_FRAGMENT_SHADER);

  std::string frag_shader_str = LoadShader("assets/glsl/test.frag");
  const GLchar* frag_shader_src = frag_shader_str.c_str();

  glShaderSource(frag_shader, 1, &frag_shader_src, nullptr);
  glCompileShader(frag_shader);

  glGetShaderiv(frag_shader, GL_COMPILE_STATUS, &success);

  if (!success)
  {
    glGetShaderInfoLog(frag_shader, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }

  unsigned int shader_prog;
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

  glUseProgram(shader_prog);

  glDeleteShader(vert_shader);
  glDeleteShader(frag_shader);
}

void RenderSystem::Initialize()
{
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);

  window = glfwCreateWindow(
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    "Last Ditch", nullptr, nullptr
  );

  if (window == nullptr)
  {
    std::cout << "Failed to create GLFW window" << std::endl;
    glfwTerminate();
    return;
  }

  glfwMakeContextCurrent(window);
  glfwSetFramebufferSizeCallback(
    window, glfw_callback::FramebufferSizeCallback
  );
  glViewport(0, 0, SCREEN_SIZE_X, SCREEN_SIZE_Y);

  glewExperimental = GL_TRUE;
  glewInit();

  CreateTestTriangle();
}

void RenderSystem::Update(const double& dt)
{
  if (glfwWindowShouldClose(window))
  {
    input.exit = true;
    return;
  }

  glClearColor(0.2f, 0.1f, 0.3f, 1.0f);
  glClear(GL_COLOR_BUFFER_BIT);
      
  glfwSwapBuffers(window);
}


std::string RenderSystem::LoadShader(const std::string& filename)
{
  std::string content;
  std::ifstream fs(filename);

  if (!fs.is_open())
  {
    std::cerr << "Could not read file " << filename << std::endl;
    return "";
  }

  std::string line("");
  while (!fs.eof())
  {
    std::getline(fs, line);
    content.append(line + "\n");
  }
  fs.close();
  return content;
}
