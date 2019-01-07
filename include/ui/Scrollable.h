#ifndef SCROLLABLE_H
#define SCROLLABLE_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "Element.h"
#include "Texture.h"
#include "Scalable.h"
#include "Scrollbar.h"
#include "../utility/Types.h"

struct Scrollable : public Element
{
  Scrollable()
    : type{}
    , texts{}
    , pos{0.0}
    , height{}
    , scroll_range{}
    , mask{}
    , content{}
    , base{}
    , scrollbar{}
  {}

  std::string type;
  std::vector<std::string> texts;

  f32 pos;

  i32 height;
  i32 scroll_range;

  SDL_Rect mask;

  Texture content;
  Scalable base;
  Scrollbar scrollbar;
};

#endif
