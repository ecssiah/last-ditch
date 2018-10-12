#include <GLFW/glfw3.h>

#include "InputSystem.h"

InputSystem::InputSystem(Input& _input, Window& _window) :
  input(_input),
  window(_window)
{
}

void InputSystem::Initialize()
{
}

void InputSystem::Update()
{
  if (glfwGetKey(window.ptr, GLFW_KEY_ESCAPE) == GLFW_PRESS)
    input.exit = true;

  if (glfwGetKey(window.ptr, GLFW_KEY_W) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

  if (glfwGetKey(window.ptr, GLFW_KEY_Q) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
}

void InputSystem::Destroy()
{
}
