#include <iostream>

#include <SDL2/SDL.h>

#include "InputSystem.h"

using namespace std;

InputSystem::InputSystem(Input& input, Render& render) 
  : input_(input)
  , render_(render)
{
}

void InputSystem::Initialize()
{
}

void InputSystem::Update()
{
  if (input_.ascend) input_.ascend = false;
  if (input_.descend) input_.descend = false;

  for(SDL_Event e; SDL_PollEvent(&e); ) {
    switch(e.type)
    {
      case SDL_QUIT:
      {
        input_.exit = true;
        break;
      }
      case SDL_KEYDOWN:
      {
        OnKeyDown(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      case SDL_KEYUP:
      {
        OnKeyUp(e.key.keysym.sym, e.key.keysym.mod, e.key.keysym.scancode);
        break;
      }
      default:
        break;
    }     
  }
}

void InputSystem::OnKeyDown(SDL_Keycode sym, Uint16 mod, Uint16 scancode)
{
  switch (sym)
  {
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
  switch (sym)
  {
    case SDLK_w: input_.up = false; break;
    case SDLK_a: input_.left = false; break;
    case SDLK_s: input_.down = false; break;
    case SDLK_d: input_.right = false; break;
    case SDLK_q: input_.min = false; break;
    case SDLK_e: input_.mag = false; break;
    default: break;
  }
}

