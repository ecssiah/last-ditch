#ifndef SCALABLE_H
#define SCALABLE_H

#include <iostream>
#include <SDL2/SDL.h>

#include "Element.h"
#include "../map/MapConstants.h"

struct Scalable : public Element 
{
  Scalable()
    : type{}
    , basex{}
    , basey{}
    , size{TILE_SIZE / 4}
    , pad{}
    , texture{nullptr}
    , src{}
    , dst{}
  {
  }

  std::string type;

  i32 basex, basey;
  i32 size;
  i32 pad;

  SDL_Texture* texture;

  std::unordered_map<std::string, SDL_Rect> src;
  std::unordered_map<std::string, SDL_Rect> dst;

};

#endif