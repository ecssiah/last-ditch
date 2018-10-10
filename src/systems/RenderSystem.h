#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <GL/glew.h>
#include <GLFW/glfw3.h>

#include "../components/Input.h"

class RenderSystem
{
public:
  RenderSystem(Input& input);
  ~RenderSystem();

  void Initialize();
  void Update(const double& dt);

private:
  Input& input;
  GLFWwindow* window;
};

#endif // RENDER_SYSTEM_H
