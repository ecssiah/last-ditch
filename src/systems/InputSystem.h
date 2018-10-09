#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include "../components/Input.h"

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
