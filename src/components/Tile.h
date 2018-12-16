#ifndef TILE_H
#define TILE_H

#include <string>
#include <SDL2/SDL.h>

#include "../constants/MapConstants.h"

struct Tile
{
  Tile()
    : active(false)
    , solid(false)
    , flip(SDL_FLIP_NONE)
    , rotation(0)
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  std::string type;
  std::string subtype;
  std::string category;

  bool active;
  bool solid;
  SDL_RendererFlip flip;

  double rotation; 

  SDL_Rect src;

};

#endif // TILE_H
