#ifndef MAP_GENERATOR_H
#define MAP_GENERATOR_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "../components/map/Map.h"
#include "../components/map/Room.h"

class MapGenerator 
{
public:
  MapGenerator(Map& map);

  void GenerateMap();

private:
  void DefineBlockedRooms(unsigned floor);
  void LayoutMainFloor(unsigned floor);
  void SeedRooms(unsigned floor);
  void ExpandRooms(unsigned floor);
  void BuildRooms(unsigned floor);
  void FinishRooms(unsigned floor);
  void IntegrateWalls(unsigned floor);
  void PlaceDoors(unsigned floor);

  bool CheckClearance(
    const std::string& category, 
    unsigned x, unsigned y, unsigned floor, unsigned direction
  );
  bool Intersects(const Room& r1, const Room& r2);
  bool Intersects(
    const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
  );
  bool RoomCollision(unsigned floor, const Room& test_room); 

  void SetTile(
    const std::string& layer, int x, int y, int floor, 
    const std::string& type, 
    float rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void SetSolid(int x, int y, int floor, bool solid);

  unsigned num_rooms_;
  unsigned expansion_iterations_;

  Map& map_;

  bool show_grid_;

  std::vector<std::vector<Room> > rooms_;
  std::vector<std::vector<Room> > blocked_rooms_; 
};

#endif
