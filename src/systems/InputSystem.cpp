#include <cmath>
#include <iostream>

#include <SDL2/SDL.h>

#include "InputSystem.h"
#include "../constants/MapConstants.h"
#include "../constants/RenderConstants.h"

using namespace std;

InputSystem::InputSystem(Input& input, Camera& camera, Render& render) 
  : input_{input}
  , camera_{camera}
  , render_{render}
{
}


void InputSystem::init()
{
}


void InputSystem::update()
{
  clear_inputs();
  call_input_functions();
  
  if (input_.lclick) calculate_selected_tile();
}


void InputSystem::clear_inputs()
{
  if (input_.ascend) input_.ascend = false;
  if (input_.descend) input_.descend = false;
  if (input_.mag) input_.mag = false;
  if (input_.min) input_.min = false;
  if (input_.lclick) input_.lclick = false;
  if (input_.mclick) input_.mclick = false;
  if (input_.rclick) input_.rclick = false;
  if (input_.lreleased) input_.lreleased = false;
  if (input_.mreleased) input_.mreleased = false;
  if (input_.rreleased) input_.rreleased = false;
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
  switch (sym) {
    case SDLK_w: input_.up = false; break;
    case SDLK_a: input_.left = false; break;
    case SDLK_s: input_.down = false; break;
    case SDLK_d: input_.right = false; break;
    case SDLK_TAB: input_.menu = !input_.menu; break;
    default: break;
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


void InputSystem::calculate_selected_tile()
{
  f32 tx{(input_.mx - HALF_SCREEN_SIZE_X) / (f32)TILE_SIZE / camera_.zoom};
  f32 ty{(input_.my - HALF_SCREEN_SIZE_Y) / (f32)TILE_SIZE / camera_.zoom};

  input_.sx = floor(tx + camera_.pos.x);
  input_.sy = floor(ty + camera_.pos.y);

  auto xcheck{input_.sx < 0 || input_.sx > TILES_PER_LAYER - 1};
  auto ycheck{input_.sy < 0 || input_.sy > TILES_PER_LAYER - 1}; 

  if (xcheck || ycheck) {
    input_.sx = -1;
    input_.sy = -1;
  }

  cout << "Tile: " << input_.sx << " " << input_.sy << endl;
}


