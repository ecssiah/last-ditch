#ifndef LAYER_H
#define LAYER_H

#include <vector>
#include <boost/serialization/access.hpp>

#include "Tile.h"
#include "../../Types.h"
#include "../../constants/MapConstants.h"

struct Layer
{
  Layer() 
    : tiles{(U16)TILES_PER_LAYER, std::vector<Tile>((U16)TILES_PER_LAYER)} 
  {}

  std::vector<std::vector<Tile> > tiles;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const U32 version)
  {
    ar & tiles;
  }
};

#endif
