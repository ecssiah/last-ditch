#ifndef RENDER_H
#define RENDER_H

#include <SDL2/SDL.h>

struct Render
{
  Render() 
    : dt{} 
    , window{nullptr}
    , renderer{nullptr}
  {}

  float dt;

  SDL_Window* window;
  SDL_Renderer* renderer;

};

#endif // RENDER_H
