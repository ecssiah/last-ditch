#ifndef SCROLLABLE_H
#define SCROLLABLE_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "Element.h"
#include "List.h"
#include "Scalable.h"
#include "Scrollbar.h"
#include "../utility/Types.h"

struct Scrollable : public Element
{
  Scrollable()
    : type{}
    , pos{0.0}
    , scroll_range{}
    , mask{}
    , list{}
    , base{}
    , scrollbar{}
  {}

  std::string type;

  f32 pos;
  i32 scroll_range;

  SDL_Rect mask;

  List list;
  Scalable base;
  Scrollbar scrollbar;

};

#endif
