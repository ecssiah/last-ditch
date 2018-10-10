#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include <GLFW/glfw3.h>

#include "../components/Input.h"

class InputSystem
{
public:
  InputSystem(Input& input);

  void Initialize(GLFWwindow* window);
  void Update();
  void Destroy();

private:
  Input& input;

  GLFWwindow* window;
};

#endif // INPUT_SYSTEM_H
