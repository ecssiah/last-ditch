#ifndef ELEMENT_H
#define ELEMENT_H

#include <string>
#include <SDL2/SDL.h>

struct Element
{
  Element()
    : id{}
    , bounds{}
    , active{true}
    , selected{false}
    , changed{false}
  {}

  virtual ~Element() = default;

  std::string id;

  SDL_Rect bounds;

  bool active;
  bool selected;
  bool changed;

};

#endif
