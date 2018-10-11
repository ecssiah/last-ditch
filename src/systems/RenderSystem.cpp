#include <iostream>
#include <vector>
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

  GLchar** shader_source = LoadShader("assets/glsl/test.vert");  
  glShaderSource(vert_shader, 1, shader_source, nullptr);
  glCompileShader(vert_shader);

  int success;
  char info_log[512];
  glGetShaderiv(vert_shader, GL_COMPILE_STATUS, &success);

  if (!success)
  {
    glGetShaderInfoLog(vert_shader, 512, nullptr, info_log);
    std::cout << info_log << std::endl;
  }
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


GLchar** RenderSystem::LoadShader(const std::string& filename)
{
  std::vector<char*> cstrings{};

  std::string line;
  std::ifstream shader_file(filename);

  if (shader_file.is_open())
  {
    while (shader_file.good())
    {
      std::getline(shader_file, line);
      cstrings.push_back(&line.front());
    }
    shader_file.close();
  } else {
    std::cout << "Failed to open shader: " << filename << std::endl;
  }

  return cstrings.data();
}
