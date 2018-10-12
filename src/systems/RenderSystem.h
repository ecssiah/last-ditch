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
  void CreateTestTriangle();
  std::string LoadShader(const std::string& filename);

  Input& input;
  Window& window;

  unsigned int shader_prog, triangle_VAO;
};

#endif // RENDER_SYSTEM_H
