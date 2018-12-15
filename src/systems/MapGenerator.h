#ifndef MAP_GENERATOR_H
#define MAP_GENERATOR_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "../components/Map.h"
#include "../components/Room.h"

class MapGenerator 
{
public:
  MapGenerator(Map& map);

  void GenerateMap(std::string name);

private:
  void SetTile(
    std::string layer, int x, int y, int floor, 
    std::string type, 
    float rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );

  Map& map_;

  std::vector<Room> rooms_;
};

#endif // MAP_GENERATOR_H
