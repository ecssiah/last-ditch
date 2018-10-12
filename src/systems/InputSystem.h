#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include <GLFW/glfw3.h>

#include "../components/Input.h"
#include "../components/Window.h"

class InputSystem
{
public:
  InputSystem(Input& input, Window& window);

  void Initialize();
  void Update();
  void Destroy();

private:
  Input& input;
  Window& window;
};

#endif // INPUT_SYSTEM_H
