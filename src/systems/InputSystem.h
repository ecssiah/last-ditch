#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include <GLFW/glfw3.h>

#include "../components/Input.h"

namespace InputCallback
{
  void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods);
}

class InputSystem
{
public:
  InputSystem(Input& input);

  void Initialize();
  void Update();
  void Destroy();

private:
  Input& input;
};

#endif // INPUT_SYSTEM_H
