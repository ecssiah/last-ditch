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
  void OnMouseDown(Sint32 x, Sint32 y, Uint8 button);
  void OnMouseUp(Sint32 x, Sint32 y, Uint8 button);

  Input& input_;
  Render& render_;
}; 

#endif // INPUT_SYSTEM_H
