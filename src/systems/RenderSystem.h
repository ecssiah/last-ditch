#ifndef RENDER_SYSTEM_H
#define RENDER_SYSTEM_H

#include <string>
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

  GLFWwindow* GetWindow() { return window; }

private:
  void SetupShaders();
  void CreateTestTriangle();
  std::string LoadShader(const std::string& filename);

  Input& input;
  GLFWwindow* window;

  unsigned int shader_prog, triangle_VAO;
};

#endif // RENDER_SYSTEM_H
