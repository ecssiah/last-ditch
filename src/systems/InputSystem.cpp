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


void InputSystem::Initialize()
{
}


void InputSystem::Update()
{
  ClearInputs();
  CallInputFunctions();
  
  if (input_.lclick) CalculateSelectedTile();
}


void InputSystem::ClearInputs()
{
  if (input_.ascend) input_.ascend = false;
  if (input_.descend) input_.descend = false;
  if (input_.mag) input_.mag = false;
  if (input_.min) input_.min = false;
  if (input_.lclick) input_.lclick = false;
  if (input_.mclick) input_.mclick = false;
  if (input_.rclick) input_.rclick = false;
}


void InputSystem::CallInputFunctions()
{
  for(SDL_Event e; SDL_PollEvent(&e); ) {
    switch(e.type) {
      case SDL_QUIT: {
        input_.exit = true;
        break;
      }
      case SDL_KEYDOWN: {
        OnKeyDown(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      case SDL_KEYUP: {
        OnKeyUp(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      case SDL_MOUSEBUTTONDOWN: {
        OnMouseDown(e.button.x, e.button.y, e.button.button);
        break;
      }
      case SDL_MOUSEBUTTONUP: {
        OnMouseUp(e.button.x, e.button.y, e.button.button);
        break;
      }
      default: break;
    }     
  }
}


void InputSystem::OnKeyDown(SDL_Keycode sym, Uint16 mod, Uint16 scancode)
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


void InputSystem::OnKeyUp(SDL_Keycode sym, Uint16 mod, Uint16 scancode)
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


void InputSystem::OnMouseDown(Sint32 x, Sint32 y, Uint8 button)
{
  input_.mx = x;
  input_.my = y;

  switch (button) {
    case SDL_BUTTON_LMASK: input_.lpressed = true; break;
    case SDL_BUTTON_MMASK: input_.mpressed = true; break;
    case SDL_BUTTON_RMASK: input_.rpressed = true; break;
    default: break;
  }
}


void InputSystem::OnMouseUp(Sint32 x, Sint32 y, Uint8 button)
{
  input_.mx = x;
  input_.my = y;

  switch (button) {
    case SDL_BUTTON_LMASK: input_.lpressed = false; input_.lclick = true; break;
    case SDL_BUTTON_MMASK: input_.mpressed = false; input_.mclick = true; break;
    case SDL_BUTTON_RMASK: input_.rpressed = false; input_.rclick = true; break;
    default: break;
  }
}


void InputSystem::CalculateSelectedTile()
{
  float tx{(input_.mx - HALF_SCREEN_SIZE_X) / (float)TILE_SIZE / camera_.zoom};
  float ty{(input_.my - HALF_SCREEN_SIZE_Y) / (float)TILE_SIZE / camera_.zoom};

  input_.sx = floor(tx + camera_.pos.x);
  input_.sy = floor(ty + camera_.pos.y);

  cout << "Tile: " << input_.sx << " " << input_.sy << endl;
}


