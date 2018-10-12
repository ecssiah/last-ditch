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
    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

  if (glfwGetKey(window_.ptr, GLFW_KEY_Q) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
}
