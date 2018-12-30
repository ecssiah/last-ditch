#ifndef MAP_GENERATOR_H
#define MAP_GENERATOR_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "../Types.h"
#include "../components/map/Map.h"
#include "../components/map/Room.h"

class MapGenerator 
{
public:
  MapGenerator(Map& map);

  void generate_map();

private:
  void layout_main_floor(I32 floor);
  void define_blocked_rooms(I32 floor);

  void seed_rooms(I32 floor);
  void expand_rooms(I32 floor);
  void build_rooms(I32 floor);
  void finish_rooms(I32 floor);
  void integrate_walls(I32 floor);
  void place_doors(I32 floor);

  bool has_clearance(
    const std::string& category, I32 x, I32 y, I32 floor, U8 direction
  );
  bool room_collision(I32 floor, const Room& test_room); 

  void set_tile(
    const std::string& layer, I32 x, I32 y, I32 floor, const std::string& type, 
    F32 rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void set_solid(I32 x, I32 y, I32 floor, bool solid);

  U16 num_rooms_;
  U16 expansion_iterations_;

  Map& map_;

  bool show_grid_, randomize_rooms_;

  std::vector<std::vector<Room> > rooms_;
  std::vector<std::vector<Room> > blocked_rooms_; 
};

#endif
