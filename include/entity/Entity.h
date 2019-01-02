#ifndef ENTITY_H
#define ENTITY_H

#include <string>
#include <SDL2/SDL.h>
#include <boost/serialization/access.hpp>

#include "../utility/Types.h"
#include "../map/MapConstants.h"

struct Entity
{
  Entity()
    : type{}
    , rotation{0}
    , src{0, 0, TILE_SIZE, TILE_SIZE}
    , flip{SDL_FLIP_NONE}
  {
  }

  std::string type;

  f64 rotation; 

  SDL_Rect src;
  SDL_RendererFlip flip;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & flip;
    ar & rotation;

    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;
  }
};

#endif
