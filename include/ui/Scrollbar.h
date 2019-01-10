#ifndef SCROLLBAR_H
#define SCROLLBAR_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>

#include "Element.h"
#include "../map/MapConstants.h"

struct Scrollbar : public Element
{
  Scrollbar()
    : type{}
    , texture{}
    , basex{}
    , basey{}
    , size{TILE_SIZE / 8}
    , src{}
    , dst{}
  {
  }

  std::string type;
  std::string texture;

  i32 basex, basey;
  i32 size;

  std::unordered_map<std::string, SDL_Rect> src;
  std::unordered_map<std::string, SDL_Rect> dst;

};

#endif