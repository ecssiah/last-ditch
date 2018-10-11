#include <GLFW/glfw3.h>

#include "InputSystem.h"

InputSystem::InputSystem(Input& _input) :
  input(_input)
{
}

void InputSystem::Initialize(GLFWwindow* _window)
{
  window = _window;
}

void InputSystem::Update()
{
  glfwPollEvents();

  if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
    glfwSetWindowShouldClose(window, true);

  if (glfwGetKey(window, GLFW_KEY_W) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

  if (glfwGetKey(window, GLFW_KEY_Q) == GLFW_PRESS)
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

  /* for (SDL_Event e; SDL_PollEvent(&e); ) */
  /* { */
  /*   switch (e.type) */
  /*   { */
  /*   case SDL_QUIT: input.exit = true; break; */
  /*   case SDL_KEYDOWN: */
  /*   { */
  /*     switch (e.key.keysym.sym) */
  /*     { */
  /*       case SDLK_w: input.up = true; break; */
  /*       case SDLK_a: input.left = true; break; */
  /*       case SDLK_s: input.down = true; break; */
  /*       case SDLK_d: input.right = true; break; */
  /*       case SDLK_ESCAPE: input.exit = true; break; */
  /*       default: break; */
  /*     } */
  /*     break; */
  /*   } */
  /*   case SDL_KEYUP: */
  /*   { */
  /*     switch (e.key.keysym.sym) */
  /*     { */
  /*       case SDLK_w: input.up = false; break; */
  /*       case SDLK_a: input.left = false; break; */
  /*       case SDLK_s: input.down = false; break; */
  /*       case SDLK_d: input.right = false; break; */
  /*       default: break; */
  /*     } */
  /*     break; */
  /*   } */
  /*   default: */
  /*      break; */
  /*   } */
  /* } */
}

void InputSystem::Destroy()
{
}
