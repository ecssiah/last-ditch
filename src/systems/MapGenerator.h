#ifndef MAP_GENERATOR_H
#define MAP_GENERATOR_H

#include "../components/Map.h"

class MapGenerator 
{
public:
  MapGenerator(Map& map);

  Map& map_;
};

#endif // MAP_GENERATOR_H
