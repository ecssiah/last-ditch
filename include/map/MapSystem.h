#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../interface/Input.h"
#include "Map.h"
#include "MapGenerator.h"

class MapSystem
{
public:
  MapSystem(Input& input, Map& map);

  void init();
  void update();

private:
  Input& input_;
  Map& map_;

  MapGenerator map_generator_;
}; 

#endif
