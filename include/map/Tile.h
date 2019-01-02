#ifndef TILE_H
#define TILE_H

#include <string>
#include <SDL2/SDL.h>
#include <glm/glm.hpp>
#include <boost/serialization/access.hpp>

#include "../utility/Types.h"
#include "MapConstants.h"

struct Tile
{
  Tile()
    : type{}
    , subtype{}
    , category{}
    , active{false}
    , solid{false}
    , rotation{0}
    , color{255, 255, 255}
    , src{0, 0, TILE_SIZE, TILE_SIZE}
    , flip{SDL_FLIP_NONE}
  {}

  std::string type;
  std::string subtype;
  std::string category;

  bool active;
  bool solid;
  f64 rotation; 

  glm::vec3 color;

  SDL_Rect src;
  SDL_RendererFlip flip;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & type;
    ar & subtype;
    ar & category;

    ar & active;
    ar & solid;
    ar & rotation;

    ar & color.x;
    ar & color.y;
    ar & color.z;

    ar & flip;

    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;
  }
};

#endif
