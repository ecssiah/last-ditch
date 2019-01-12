#ifndef INPUT_SYSTEM_H
#define INPUT_SYSTEM_H

#include "Input.h"
#include "../utility/Types.h"
#include "../render/Camera.h"
#include "../render/Render.h"

class InputSystem
{
public:
  InputSystem(Input& input, Render& render, Camera& camera);

  void init();
  void update();

private:
  void on_multigesture(SDL_MultiGestureEvent e);

  void on_finger_up(SDL_TouchFingerEvent e);
  void on_finger_down(SDL_TouchFingerEvent e);
  void on_finger_motion(SDL_TouchFingerEvent e);

  void on_key_down(SDL_Keycode sym, u16 mod, u16 scancode);
  void on_key_up(SDL_Keycode sym, u16 mod, u16 scancode);

  void on_mouse_down(i32 x, i32 y, u8 button);
  void on_mouse_up(i32 x, i32 y, u8 button);
  void on_mouse_motion(SDL_MouseMotionEvent motion, SDL_MouseButtonEvent button);

  void clear_inputs();
  void call_input_functions();

  Input& input_;
  Render& render_;
  Camera& camera_;

}; 

#endif
