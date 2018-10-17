#ifndef RENDER_H
#define RENDER_H

#include <GLFW/glfw3.h>

struct Render
{
  Render() 
    : window(nullptr)
  {}

  float dt;

  GLFWwindow* window;
};

#endif // RENDER_H
