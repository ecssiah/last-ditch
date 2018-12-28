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

  void generate_map();

private:
  void define_blocked_rooms(unsigned floor);
  void layout_main_floor(unsigned floor);
  void seed_rooms(unsigned floor);
  void expand_rooms(unsigned floor);
  void build_rooms(unsigned floor);
  void finish_rooms(unsigned floor);
  void integrate_walls(unsigned floor);
  void place_doors(unsigned floor);

  bool check_clearance(
    const std::string& category, 
    unsigned x, unsigned y, unsigned floor, unsigned direction
  );
  bool intersects(const Room& r1, const Room& r2);
  bool intersects(
    const Room& r1, unsigned l, unsigned r, unsigned t, unsigned b
  );
  bool room_collision(unsigned floor, const Room& test_room); 

  void set_tile(
    const std::string& layer, int x, int y, int floor, 
    const std::string& type, 
    float rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void set_solid(int x, int y, int floor, bool solid);

  unsigned num_rooms_;
  unsigned expansion_iterations_;

  Map& map_;

  bool show_grid_;

  std::vector<std::vector<Room> > rooms_;
  std::vector<std::vector<Room> > blocked_rooms_; 
};

#endif
