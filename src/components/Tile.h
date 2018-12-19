#ifndef TILE_H
#define TILE_H

#include <string>
#include <SDL2/SDL.h>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "../constants/MapConstants.h"

struct Tile
{
  Tile()
    : flip(SDL_FLIP_NONE)
  {
    src.w = TILE_SIZE; 
    src.h = TILE_SIZE;
  }

  std::string type;
  std::string subtype;
  std::string category;

  bool active;
  bool solid;

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
    ar & flip;

    ar & rotation;

    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;
  }
};

#endif // TILE_H
