#ifndef TILE_H
#define TILE_H

#include <string>
#include <SDL2/SDL.h>
#include <glm/glm.hpp>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "../constants/MapConstants.h"

struct Tile
{
  Tile()
    : flip(SDL_FLIP_NONE)
    , color(255, 255, 255)
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  std::string type;
  std::string subtype;
  std::string category;

  bool active;
  bool solid;

  glm::vec3 color;

  SDL_Rect src;
  SDL_RendererFlip flip;

  double rotation; 

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int version)
  {
    ar & type;
    ar & subtype;
    ar & category;

    ar & active;
    ar & solid;

    ar & color.x;
    ar & color.y;
    ar & color.z;

    ar & flip;
    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;

    ar & rotation;
  }
};

#endif // TILE_H
