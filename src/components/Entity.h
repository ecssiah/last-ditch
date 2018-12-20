#ifndef ENTITY_H
#define ENTITY_H

#include <string>
#include <SDL2/SDL.h>
#include <boost/archive/binary_oarchive.hpp>

#include "../constants/MapConstants.h"

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

  double rotation; 

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int version)
  {
    ar & flip;

    ar & src.x;
    ar & src.y;
    ar & src.w;
    ar & src.h;

    ar & rotation;
  }
};

#endif // ENTITY_H
