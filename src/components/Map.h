#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

#include "Floor.h"

struct Map
{
  Map() 
    : cur_floor(0)
    , floors()
    , attributes()
  {
  }

  unsigned cur_floor;

  std::vector<Floor> floors;
  std::vector<float> attributes;
};

#endif // MAP_H
