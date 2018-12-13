#ifndef TILE_H
#define TILE_H

#include <SDL2/SDL.h>

#include "../constants/MapConstants.h"

struct Tile
{
  Tile()
    : type(0)
    , category(0)
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  unsigned category;
  unsigned type;

  SDL_Rect src;

};

#endif // TILE_H
