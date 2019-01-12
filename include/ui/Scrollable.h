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
    : base_type{}
    , scrollbar_type{}
    , list_font{}
    , list_items{}
    , pos{0.0}
    , pad{}
    , scroll_range{}
    , mask{}
    , list{}
    , base{}
    , scrollbar{}
  {}

  std::string base_type;
  std::string scrollbar_type;
  std::string list_font;

  std::vector<std::string> list_items;

  f32 pos;
  SDL_Point pad;
  i32 scroll_range;

  SDL_Rect mask;

  List list;
  Scalable base;
  Scrollbar scrollbar;

};

#endif
