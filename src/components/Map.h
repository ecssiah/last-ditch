#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

#include "Layer.h"

struct Map
{
  Map() 
    : layers()
    , attributes()
  {
  }

  std::unordered_map<std::string, Layer> layers;
  std::vector<float> attributes;
};

#endif // MAP_H
