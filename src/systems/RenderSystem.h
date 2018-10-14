#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <GLFW/glfw3.h>

#include "../components/Input.h"
#include "../components/Window.h"
#include "../components/Camera.h"

class RenderSystem
{
public:
  RenderSystem(Input& input, Window& window, Camera& camera);
  ~RenderSystem();

  void Initialize();
  void Update();

private:
  void RunTests();

  Input& input_;
  Window& window_;
  Camera& camera_;

  unsigned int shader_prog_, VAO_;
  unsigned int texture0_, texture1_;
};

#endif // RENDER_SYSTEM_H
