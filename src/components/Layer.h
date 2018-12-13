#ifndef LAYER_H
#define LAYER_H

#include <vector>

#include "Tile.h"
#include "../constants/MapConstants.h"

struct Layer
{
  Layer() 
    : tiles(TILES_PER_LAYER, std::vector<Tile>(TILES_PER_LAYER)) 
  {}

  std::vector<std::vector<Tile> > tiles;

};

#endif // LAYER_H
