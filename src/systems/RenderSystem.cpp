#include <iostream>
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

  LoadShader("assets/glsl/test.vert");  
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


const GLchar** RenderSystem::LoadShader(const std::string& filename)
{
  std::string line;
  std::ifstream shader_file(filename);

  if (shader_file.is_open())
  {
    while (shader_file.good())
    {
      std::getline(shader_file, line);
      std::cout << line << std::endl;
    }
    shader_file.close();
  } else {
    std::cout << "Failed to open shader: " << filename << std::endl;
  }

  return nullptr;
}
