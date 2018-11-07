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
  void OnKeyDown(SDL_Keycode sym, Uint16 mod, Uint16 scancode);
  void OnKeyUp(SDL_Keycode sym, Uint16 mod, Uint16 scancode);

  Input& input_;
  Render& render_;
}; 

#endif // INPUT_SYSTEM_H
