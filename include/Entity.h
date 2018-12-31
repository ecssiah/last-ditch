#ifndef ENTITY_H
#define ENTITY_H

#include <string>
#include <SDL2/SDL.h>
#include <boost/serialization/access.hpp>

#include "Types.h"
#include "map/MapConstants.h"

struct Entity
{
  Entity()
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  std::string type;

  SDL_Rect src;
  SDL_RendererFlip flip;

  f64 rotation; 

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & flip;

    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;

    ar & rotation;
  }
};

#endif
