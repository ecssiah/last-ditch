#ifndef WINDOW_H
#define WINDOW_H

#include <GLFW/glfw3.h>

struct Window
{
  Window() : 
    ptr(nullptr)
  {}

  GLFWwindow* ptr;
};

#endif // WINDOW_H
