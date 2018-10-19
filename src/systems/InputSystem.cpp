#include <iostream>

#include "InputSystem.h"

using namespace std;

InputSystem::InputSystem(Input& input, Render& render) 
  : input_(input)
  , render_(render)
{
}

void InputSystem::Initialize()
{
  glfwSetWindowUserPointer(render_.window, this);
  glfwSetKeyCallback(render_.window, key_callback);
  glfwSetCursorPosCallback(render_.window, cursor_position_callback);
  glfwSetMouseButtonCallback(render_.window, mouse_button_callback);
}

void InputSystem::Update()
{
  if (glfwWindowShouldClose(render_.window)) input_.exit = true;
}

void InputSystem::KeyCallback(int key, int scancode, int action, int mods) {
  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
    input_.exit = true;
  if (key == GLFW_KEY_T && action == GLFW_RELEASE)
    input_.debug = !input_.debug;

  if (key == GLFW_KEY_W && action == GLFW_PRESS)
    input_.up = true;
  if (key == GLFW_KEY_W && action == GLFW_RELEASE)
    input_.up = false;
  if (key == GLFW_KEY_A && action == GLFW_PRESS)
    input_.left = true;
  if (key == GLFW_KEY_A && action == GLFW_RELEASE)
    input_.left = false;
  if (key == GLFW_KEY_S && action == GLFW_PRESS)
    input_.down = true;
  if (key == GLFW_KEY_S && action == GLFW_RELEASE)
    input_.down = false;
  if (key == GLFW_KEY_D && action == GLFW_PRESS)
    input_.right = true;
  if (key == GLFW_KEY_D && action == GLFW_RELEASE)
    input_.right = false;
  if (key == GLFW_KEY_Q && action == GLFW_PRESS)
    input_.min = true;
  if (key == GLFW_KEY_Q && action == GLFW_RELEASE)
    input_.min = false;
  if (key == GLFW_KEY_E && action == GLFW_PRESS)
    input_.mag = true;
  if (key == GLFW_KEY_E && action == GLFW_RELEASE)
    input_.mag = false;
}

void InputSystem::CursorPosCallback(double xpos, double ypos) 
{
}

void InputSystem::MouseButtonCallback(int button, int action, int mods)
{
  glfwGetCursorPos(render_.window, &input_.mx, &input_.my);

  if (button == GLFW_MOUSE_BUTTON_LEFT)
    input_.lclick = action == GLFW_PRESS ? true : false;
  if (button == GLFW_MOUSE_BUTTON_RIGHT)
    input_.rclick = action == GLFW_PRESS ? true : false;
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

void InputSystem::mouse_button_callback(
  GLFWwindow* window, int button, int action, int mods
) {
  auto input_system = (InputSystem*)glfwGetWindowUserPointer(window);
  input_system->MouseButtonCallback(button, action, mods);
}
