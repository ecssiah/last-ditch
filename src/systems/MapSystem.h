#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../components/Map.h"

class MapSystem
{
public:
  MapSystem(Map& map);

  void Initialize();
  void Update();

private:
  void GenerateMap();

  void SetTile(std::string layer, int x, int y, int type);

  Map& map_;
}; 

#endif // MAP_SYSTEM_H
