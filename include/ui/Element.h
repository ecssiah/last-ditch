#ifndef ELEMENT_H
#define ELEMENT_H

#include <SDL2/SDL.h>

struct Element
{
  Element()
    : rect{}
  {}

  SDL_Rect rect;

};

#endif
