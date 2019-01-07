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
    , basex{}
    , basey{}
    , size{TILE_SIZE / 8}
    , texture{nullptr}
    , src{}
    , dst{}
  {
  }

  std::string type;

  i32 basex, basey;
  i32 size;

  SDL_Texture* texture;

  std::unordered_map<std::string, SDL_Rect> src;
  std::unordered_map<std::string, SDL_Rect> dst;

};

#endif