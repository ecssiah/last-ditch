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
    , rot{0}
    , dst{0, 0, TILE_SIZE, TILE_SIZE}
    , flip{SDL_FLIP_NONE}
  {
  }

  std::string type;

  f32 rot; 

  SDL_Rect dst;
  SDL_RendererFlip flip;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & flip;
    ar & rot;

    ar & dst.x;
    ar & dst.y;
    ar & dst.w;
    ar & dst.h;
  }
};

#endif
