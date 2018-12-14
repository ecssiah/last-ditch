#ifndef FLOOR_H
#define FLOOR_H

#include <string>
#include <unordered_map>

#include "Layer.h"

struct Floor
{
  Floor()
    :layers()
  {}

  std::unordered_map<std::string, Layer> layers;
};

#endif // FLOOR_H
