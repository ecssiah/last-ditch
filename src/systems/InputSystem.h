#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include "../Types.h"
#include "../components/Input.h"
#include "../components/Camera.h"
#include "../components/Render.h"

class InputSystem
{
public:
  InputSystem(Input& input, Camera& camera, Render& render);

  void init();
  void update();

private:
  void on_key_down(SDL_Keycode sym, u16 mod, u16 scancode);
  void on_key_up(SDL_Keycode sym, u16 mod, u16 scancode);
  void on_mouse_down(Sint32 x, i32 y, u8 button);
  void on_mouse_up(Sint32 x, i32 y, u8 button);

  void clear_inputs();
  void call_input_functions();
  void calculate_selected_tile();

  Input& input_;
  Camera& camera_;
  Render& render_;
}; 

#endif
