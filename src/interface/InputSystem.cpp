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
}


void InputSystem::call_input_functions()
{
  for(SDL_Event e; SDL_PollEvent(&e); ) {
    switch(e.type) {
      case SDL_QUIT: {
        input_.exit = true;
        break;
      }
      case SDL_KEYDOWN: {
        on_key_down(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      case SDL_KEYUP: {
        on_key_up(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      case SDL_MOUSEBUTTONDOWN: {
        on_mouse_down(e.button.x, e.button.y, e.button.button);
        break;
      }
      case SDL_MOUSEBUTTONUP: {
        on_mouse_up(e.button.x, e.button.y, e.button.button);
        break;
      }
      case SDL_MOUSEMOTION: {
        on_mouse_motion(e.motion.xrel, e.motion.yrel, e.button.button);
        break;
      }
      default: break;
    }     
  }
}


void InputSystem::on_key_down(SDL_Keycode sym, u16 mod, u16 scancode)
{
  switch (sym) {
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


void InputSystem::on_key_up(SDL_Keycode sym, u16 mod, u16 scancode)
{
  if (mod == KMOD_NONE) {
    switch (sym) {
      case SDLK_w: input_.up = false; break;
      case SDLK_a: input_.left = false; break;
      case SDLK_s: input_.down = false; break;
      case SDLK_d: input_.right = false; break;
      case SDLK_TAB: input_.menu = !input_.menu; break;
      default: break;
    }
  } else if (mod == KMOD_RSHIFT) {
    switch (sym) {
      case SDLK_TAB: input_.hud = !input_.hud; break;
      default: break;
    }
  }
}


void InputSystem::on_mouse_down(i32 x, i32 y, u8 button)
{
  input_.mx = x;
  input_.my = y;

  switch (button) {
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


void InputSystem::on_mouse_up(i32 x, i32 y, u8 button)
{
  input_.mx = x;
  input_.my = y;

  switch (button) {
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


void InputSystem::on_mouse_motion(i32 xrel, i32 yrel, u8 button)
{
  input_.mdx = xrel;
  input_.mdy = yrel;
}

