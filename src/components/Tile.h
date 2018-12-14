#ifndef TILE_H
#define TILE_H

#include <SDL2/SDL.h>

#include "../constants/MapConstants.h"

struct Tile
{
  Tile()
    : active(false)
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  bool active;

  SDL_Rect src;

};

#endif // TILE_H
