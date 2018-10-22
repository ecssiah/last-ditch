#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../components/Chunk.h"
#include "../components/Map.h"

class MapSystem
{
public:
  MapSystem(Map& map);

  void Initialize();
  void Update();

private:
  void GenerateMap();

  Map& map_;
}; 

#endif // MAP_SYSTEM_H
