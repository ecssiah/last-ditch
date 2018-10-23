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
  {
  }

  std::unordered_map<std::string, Layer> layers;

};

#endif // MAP_H
