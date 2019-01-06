#ifndef SCROLLABLE_H
#define SCROLLABLE_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "Element.h"
#include "Scalable.h"
#include "../utility/Types.h"

struct Scrollable : public Element
{
  Scrollable()
    : type{}
    , texts{}
    , pos{}
    , mask{}
    , texture{nullptr}
    , base{}
    , scrollbar{}
  {}

  std::string type;
  std::vector<std::string> texts;

  f32 pos;

  SDL_Rect mask;
  SDL_Texture* texture;

  Scalable base;
  Scalable scrollbar;
};

#endif
