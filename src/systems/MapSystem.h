#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../components/Input.h"
#include "../components/map/Map.h"
#include "MapGenerator.h"

class MapSystem
{
public:
  MapSystem(Input& input, Map& map);

  void Initialize();
  void Update();

private:
  Input& input_;
  Map& map_;

  MapGenerator map_generator_;
}; 

#endif
