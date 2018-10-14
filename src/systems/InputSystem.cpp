#include <iostream>

#include "InputSystem.h"

InputSystem::InputSystem(Input& input, Window& window) 
  : input_(input)
  , window_(window)
{
}

void InputSystem::Initialize()
{
  glfwSetWindowUserPointer(window_.ptr, this);
  glfwSetKeyCallback(window_.ptr, key_callback);
  glfwSetCursorPosCallback(window_.ptr, cursor_position_callback);
}

void InputSystem::Update()
{
}

void InputSystem::KeyCallback(int key, int scancode, int action, int mods) {
  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
    input_.exit = true;
  if (key == GLFW_KEY_W && action == GLFW_PRESS)
    input_.up = true;
  if (key == GLFW_KEY_A && action == GLFW_PRESS)
    input_.left = true;
  if (key == GLFW_KEY_S && action == GLFW_PRESS)
    input_.down = true;
  if (key == GLFW_KEY_D && action == GLFW_PRESS)
    input_.right = true;
  if (key == GLFW_KEY_W && action == GLFW_RELEASE)
    input_.up = false;
  if (key == GLFW_KEY_A && action == GLFW_RELEASE)
    input_.left = false;
  if (key == GLFW_KEY_S && action == GLFW_RELEASE)
    input_.down = false;
  if (key == GLFW_KEY_D && action == GLFW_RELEASE)
    input_.right = false;
  if (key == GLFW_KEY_T && action == GLFW_RELEASE)
    input_.debug = !input_.debug;
}

void InputSystem::CursorPosCallback(double xpos, double ypos) 
{
}

void InputSystem::key_callback(
  GLFWwindow* window, int key, int scancode, int action, int mods
) {
  auto input_system = (InputSystem*)glfwGetWindowUserPointer(window);
  input_system->KeyCallback(key, scancode, action, mods);
}

void InputSystem::cursor_position_callback(
  GLFWwindow* window, double xpos, double ypos
) {
  auto input_system = (InputSystem*)glfwGetWindowUserPointer(window);
  input_system->CursorPosCallback(xpos, ypos);
}

