#include <GLFW/glfw3.h>

#include "InputSystem.h"

InputSystem::InputSystem(Input& input, Window& window) 
  : input_(input)
  , window_(window)
{
}

void InputSystem::Initialize()
{
}

void InputSystem::Update()
{
  if (glfwGetKey(window_.ptr, GLFW_KEY_ESCAPE) == GLFW_PRESS)
    input_.exit = true;

  if (glfwGetKey(window_.ptr, GLFW_KEY_W) == GLFW_PRESS)
    input_.up = true;
  if (glfwGetKey(window_.ptr, GLFW_KEY_A) == GLFW_PRESS)
    input_.left = true;
  if (glfwGetKey(window_.ptr, GLFW_KEY_S) == GLFW_PRESS)
    input_.down = true;
  if (glfwGetKey(window_.ptr, GLFW_KEY_D) == GLFW_PRESS)
    input_.right = true;
  if (glfwGetKey(window_.ptr, GLFW_KEY_W) == GLFW_RELEASE)
    input_.up = false;
  if (glfwGetKey(window_.ptr, GLFW_KEY_A) == GLFW_RELEASE)
    input_.left = false;
  if (glfwGetKey(window_.ptr, GLFW_KEY_S) == GLFW_RELEASE)
    input_.down = false;
  if (glfwGetKey(window_.ptr, GLFW_KEY_D) == GLFW_RELEASE)
    input_.right = false;

  if (glfwGetKey(window_.ptr, GLFW_KEY_R) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
  if (glfwGetKey(window_.ptr, GLFW_KEY_T) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
}
