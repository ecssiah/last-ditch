#ifndef LAYER_H
#define LAYER_H

#include <vector>

#include "Chunk.h"
#include "../constants/MapConstants.h"

struct Layer
{
  Layer() 
    : chunks(CHUNKS_PER_LAYER, std::vector<Chunk>(CHUNKS_PER_LAYER)) 
  {}

  std::vector<std::vector<Chunk> > chunks;

};

#endif // LAYER_H
