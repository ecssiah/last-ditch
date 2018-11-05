#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include "../components/Input.h"
#include "../components/Render.h"

class InputSystem
{
public:
  InputSystem(Input& input, Render& render);

  void Initialize();
  void Update();

private:
  Input& input_;
  Render& render_;
}; 

#endif // INPUT_SYSTEM_H
