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
  void layout_main_floor(i32 floor);
  void define_blocked_rooms(i32 floor);

  void seed_rooms(i32 floor);
  void expand_rooms(i32 floor);
  void build_rooms(i32 floor);
  void finish_rooms(i32 floor);
  void integrate_walls(i32 floor);
  void place_doors(i32 floor);

  bool has_clearance(
    const std::string& category, i32 x, i32 y, i32 floor, u8 direction
  );
  bool room_collision(i32 floor, const Room& test_room); 

  void set_tile(
    const std::string& layer, i32 x, i32 y, i32 floor, const std::string& type, 
    f32 rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void set_solid(i32 x, i32 y, i32 floor, bool solid);

  Map& map_;

  bool show_grid_;
  bool randomize_rooms_;

  u16 num_rooms_;
  u16 expansion_iterations_;

  std::vector<std::vector<Room> > rooms_;
  std::vector<std::vector<Room> > blocked_rooms_; 
};

#endif
