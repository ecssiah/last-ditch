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
    , items{}
    , pos{0.0}
    , height{}
    , scroll_range{}
    , mask{}
    , body{}
    , base{}
    , scrollbar{}
  {}

  std::string type;
  std::vector<std::string> items;

  f32 pos;
  i32 height;
  i32 scroll_range;

  SDL_Rect mask;

  List body;
  Scalable base;
  Scrollbar scrollbar;

};

#endif
