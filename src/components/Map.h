#ifndef MAP_H
#define MAP_H

#include <vector>

#include "Chunk.h"
#include "../constants/MapConstants.h"

struct Map
{
  Map() 
    : chunks(CHUNKS_PER_MAP, std::vector<Chunk>(CHUNKS_PER_MAP))
  {}

  std::vector<std::vector<Chunk> > chunks;
};

#endif // MAP_H
