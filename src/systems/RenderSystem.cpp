#include <iostream>
#include <functional>
#define STB_IMAGE_IMPLEMENTATION
#include "../ext/stb/stb_image.h"

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"

RenderSystem::RenderSystem(Input& _input) :
  input(_input),
  window(nullptr)
{
}

RenderSystem::~RenderSystem()
{
  std::cout << "Render System Shutdown" << std::endl;
}

void FramebufferSizeCallback(
  GLFWwindow* window, int width, int height
) {
  glViewport(0, 0, width, height);
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

  /* std::tr1::function<void()> callback; */ 
  /* callback = std::tr1::bind(&RenderSystem::FramebufferSizeCallback, &this); */

  glfwSetFramebufferSizeCallback(window, FramebufferSizeCallback);

  glewExperimental = GL_TRUE;
  glewInit();

  glViewport(0, 0, SCREEN_SIZE_X, SCREEN_SIZE_Y);

}

void RenderSystem::Update(const double& dt)
{
  if (glfwWindowShouldClose(window))
  {
    input.exit = true;
    return;
  }
      
  glfwSwapBuffers(window);
}

