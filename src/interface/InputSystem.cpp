#include <cmath>
#include <iostream>

#include <SDL2/SDL.h>

#include "../../include/utility/Logging.h"
#include "../../include/interface/InputSystem.h"
#include "../../include/map/MapConstants.h"
#include "../../include/render/RenderConstants.h"

using namespace std;

InputSystem::InputSystem(Input& input, Render& render, Camera& camera) 
  : input_{input}
  , render_{render}
  , camera_{camera}
{
}


void InputSystem::init()
{
}


void InputSystem::update()
{
  clear_inputs();
  call_input_functions();
}


void InputSystem::clear_inputs()
{
  input_.ascend = false;
  input_.descend = false;
  input_.mag = false;
  input_.min = false;
  input_.lclick = false;
  input_.mclick = false;
  input_.rclick = false;
  input_.lreleased = false;
  input_.mreleased = false;
  input_.rreleased = false;

  input_.mdx = 0;
  input_.mdy = 0;

  input_.tx = 0;
  input_.ty = 0;
  input_.tdx = 0;
  input_.tdy = 0;

  input_.touch_points = 0;
}


void InputSystem::call_input_functions()
{
  for(SDL_Event e; SDL_PollEvent(&e); ) {
    switch(e.type) {
      case SDL_KEYDOWN:         on_key_down(e.key); break;
      case SDL_KEYUP:           on_key_up(e.key); break;
      case SDL_MOUSEBUTTONDOWN: on_mouse_down(e.button); break;
      case SDL_MOUSEBUTTONUP:   on_mouse_up(e.button); break;
      case SDL_MOUSEMOTION:     on_mouse_motion(e.motion); break;
      case SDL_FINGERDOWN:      on_finger_down(e.tfinger); break;
      case SDL_FINGERUP:        on_finger_up(e.tfinger); break;
      case SDL_FINGERMOTION:    on_finger_motion(e.tfinger); break;
      case SDL_MULTIGESTURE:    on_multigesture(e.mgesture); break;
      case SDL_QUIT:            on_quit(); break;
      default: break;
    }
  }
}


void InputSystem::on_quit()
{
  input_.exit = true;
}


void InputSystem::on_multigesture(SDL_MultiGestureEvent e)
{
  input_.touch_points = e.numFingers;
}


void InputSystem::on_finger_down(SDL_TouchFingerEvent e)
{

}


void InputSystem::on_finger_up(SDL_TouchFingerEvent e)
{

}


void InputSystem::on_finger_motion(SDL_TouchFingerEvent e)
{
  input_.tx = e.x;
  input_.ty = e.y;
  input_.tdx = e.dx;
  input_.tdy = e.dy;
}


void InputSystem::on_key_down(SDL_KeyboardEvent key)
{
  switch (key.keysym.sym) {
    case SDLK_w: input_.up = true; break;
    case SDLK_a: input_.left = true; break;
    case SDLK_s: input_.down = true; break;
    case SDLK_d: input_.right = true; break;
    case SDLK_q: input_.min = true; break;
    case SDLK_e: input_.mag = true; break;
    case SDLK_r: input_.ascend = true; break;
    case SDLK_f: input_.descend = true; break;
    case SDLK_ESCAPE: input_.exit = true; break;
    default: break;
  }
}


void InputSystem::on_key_up(SDL_KeyboardEvent key)
{
  if (key.keysym.mod == KMOD_NONE) {
    switch (key.keysym.sym) {
      case SDLK_w: input_.up = false; break;
      case SDLK_a: input_.left = false; break;
      case SDLK_s: input_.down = false; break;
      case SDLK_d: input_.right = false; break;
      case SDLK_TAB: input_.menu = !input_.menu; break;
      default: break;
    }
  } else if (key.keysym.mod == KMOD_RSHIFT) {
    switch (key.keysym.sym) {
      case SDLK_TAB: input_.hud = !input_.hud; break;
      default: break;
    }
  }
}


void InputSystem::on_mouse_down(SDL_MouseButtonEvent mouse)
{
  input_.mx = mouse.x;
  input_.my = mouse.y;

  switch (mouse.button) {
    case SDL_BUTTON_LMASK: {
      input_.lclick = true;
      input_.lpressed = true; 
      break;
    }
    case SDL_BUTTON_MMASK: {
      input_.mclick = true;
      input_.mpressed = true; 
      break;
    }
    case SDL_BUTTON_RMASK: {
      input_.rclick = true;
      input_.rpressed = true; 
      break;
    }
    default: break;
  }
}


void InputSystem::on_mouse_up(SDL_MouseButtonEvent mouse)
{
  input_.mx = mouse.x;
  input_.my = mouse.y;

  switch (mouse.button) {
    case SDL_BUTTON_LMASK: {
      input_.lreleased = true;
      input_.lpressed = false; 
      break;
    }
    case SDL_BUTTON_MMASK: {
      input_.mreleased = true;
      input_.mpressed = false; 
      break;
    }
    case SDL_BUTTON_RMASK: {
      input_.rreleased = true;
      input_.rpressed = false; 
      break;
    }
    default: break;
  }
}


void InputSystem::on_mouse_motion(SDL_MouseMotionEvent motion) 
{
  input_.mx = motion.x;
  input_.my = motion.y;

  input_.mdx = motion.xrel;
  input_.mdy = motion.yrel;
}

