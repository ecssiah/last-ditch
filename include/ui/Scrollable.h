#ifndef SCROLLABLE_H
#define SCROLLABLE_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "Element.h"
#include "../utility/Types.h"

struct Scrollable : public Element
{
  Scrollable()
    : type{}
    , texts{}
    , pos{}
    , texture{nullptr}
  {}

  std::string type;
  std::vector<std::string> texts;

  f32 pos;

  SDL_Texture* texture;
};

#endif
