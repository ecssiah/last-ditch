#ifndef ELEMENT_H
#define ELEMENT_H

#include <SDL2/SDL.h>

struct Element
{
  Element()
    : bounds{}
    , selected{false}
    , changed{false}
  {}

  virtual ~Element() = default;

  SDL_Rect bounds;

  bool selected;
  bool changed;

};

#endif
