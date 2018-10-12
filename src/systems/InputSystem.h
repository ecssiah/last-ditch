#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include "../components/Input.h"
#include "../components/Window.h"

class InputSystem
{
public:
  InputSystem(Input& input, Window& window);

  void Initialize();
  void Update();

private:
  Input& input_;
  Window& window_;
};

#endif // INPUT_SYSTEM_H
