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

  void SeedRooms(unsigned floor, unsigned num_floors);
  void ExpandRooms(unsigned floor);
  void BuildRooms(unsigned floor);

  bool Intersects(const Room& r1, const Room& r2);
  bool Intersects(
    const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
  );

  Map& map_;

  std::vector<std::vector<Room> > rooms_;
};

#endif // MAP_GENERATOR_H
