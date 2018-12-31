#ifndef LAYER_H
#define LAYER_H

#include <vector>
#include <boost/serialization/access.hpp>

#include "Tile.h"
#include "../Types.h"
#include "MapConstants.h"

struct Layer
{
  Layer() 
    : tiles{(u32)TILES_PER_LAYER, std::vector<Tile>((u32)TILES_PER_LAYER)} 
  {}

  std::vector<std::vector<Tile> > tiles;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & tiles;
  }
};

#endif
