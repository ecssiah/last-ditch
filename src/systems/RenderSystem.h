#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <GLFW/glfw3.h>

#include "../components/Input.h"
#include "../components/Window.h"

class RenderSystem
{
public:
  RenderSystem(Input& input, Window& window);
  ~RenderSystem();

  void Initialize();
  void Update(const double& dt);

private:
  static void FrameBufferSizeCallback(GLFWwindow* window, int w, int h); 

  void SetupShaders();
  void RunTests();
  std::string LoadShader(const std::string& filename);

  Input& input_;
  Window& window_;

  unsigned int shader_prog_, VAO_;
  unsigned int texture0_, texture1_;
};

#endif // RENDER_SYSTEM_H
