#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
#include <GLFW/glfw3.h>

#include "../components/Input.h"

namespace RenderCallbacks
{
  void frame_buffer_size_callback(GLFWwindow* window, int width, int height);
}

class RenderSystem
{
public:
  RenderSystem(Input& input);
  ~RenderSystem();

  void Initialize();
  void Update(const double& dt);

private:
  void SetupShaders();
  void CreateTestTriangle();
  std::string LoadShader(const std::string& filename);

  Input& input;
  GLFWwindow* window;

  unsigned int shader_prog, triangle_VAO;
};

#endif // RENDER_SYSTEM_H
