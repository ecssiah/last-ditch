#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../components/Input.h"
#include "../components/Map.h"
#include "MapGenerator.h"

class MapSystem
{
public:
  MapSystem(Input& input, Map& map);

  void Initialize();
  void Update();

  void SaveMap(std::string filename);
  bool LoadMap(std::string filename);

private:
  void GenerateMap();

  void SetTile(
    std::string layer, 
    int x, int y, int floor, 
    std::string type, float rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );

  Input& input_;
  Map& map_;

  MapGenerator map_generator_;
}; 

#endif // MAP_SYSTEM_H
