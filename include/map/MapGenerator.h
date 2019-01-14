#ifndef MAP_GENERATOR_H
#define MAP_GENERATOR_H

#include <string>
#include <vector>
#include <SDL2/SDL.h>

#include "Map.h"
#include "Room.h"
#include "MapConstants.h"
#include "../utility/Types.h"

class MapGenerator 
{
public:
  MapGenerator(Map& map);

  void generate_map();

  inline bool has_overlay() { return show_grid_; }

  void set_tile(
    const std::string& layer, i32 x, i32 y, i32 floor, 
    const std::string& type, 
    f32 rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void set_active(
    const std::string& layer, i32 x, i32 y, i32 floor, bool active = true
  );
  void set_solid(
    i32 x, i32 y, i32 floor, bool solid = true
  );

private:
  void layout_main_floor(i32 floor);
  void define_blocked_rooms(i32 floor);

  void seed_rooms(i32 floor);
  void expand_rooms(i32 floor);
  void build_rooms(i32 floor);
  void integrate_walls(i32 floor);
  void place_doors(i32 floor);

  void set_overlay();

  bool has_clearance(
    const std::string& category, i32 x, i32 y, i32 floor, Dir dir
  );
  bool room_collision(i32 floor, const Room& test_room); 

  const i32 get_section(i32 floor);

  Map& map_;

  bool show_grid_;
  bool randomize_rooms_;

  u16 num_rooms_;
  u16 expansion_iterations_;

  std::vector<std::vector<Room> > rooms_;
  std::vector<std::vector<Room> > blocked_rooms_; 
};

#endif
