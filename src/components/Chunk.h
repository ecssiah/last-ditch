#ifndef CHUNK_H
#define CHUNK_H

#include <vector>

#include "Tile.h"
#include "../constants/MapConstants.h"

struct Chunk
{
  Chunk() 
    : tiles(TILES_PER_CHUNK, std::vector<Tile>(TILES_PER_CHUNK))
  {
  }

  std::vector<std::vector<Tile> > tiles;
};

#endif // CHUNK_H
