#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "Map.h"
#include "Room.h"
#include "../render/Camera.h"
#include "../utility/Log.h"
#include "../interface/Input.h"

class MapSystem
{
public:
  MapSystem(Input& input, Camera& camera, Map& map, Log& log);

  void init();
  void update();

private:
  void generate_map();
  void update_floor();

  void layout_main_floor(i32 floor);
  void define_blocks(i32 floor);
  void seed_rooms(i32 floor);
  void expand_rooms(i32 floor);
  void build_rooms(i32 floor);
  void integrate_walls(i32 floor);
  void place_doors(i32 floor);

  void set_tile(
    const std::string& layer, i32 x, i32 y, i32 floor, const std::string& type, 
    f32 rotation = 0, SDL_RendererFlip flip = SDL_FLIP_NONE
  );
  void set_active(
    const std::string& layer, i32 x, i32 y, i32 floor, bool active = true
  );
  void set_solid(
    i32 x, i32 y, i32 floor, bool solid = true
  );

  void clear_selection();
  void calculate_selected_tile();

  const bool select_tile(i32 x, i32 y);

  const i32 get_section(i32 floor) const;

  const bool has_clearance(
    const std::string& category, i32 x, i32 y, i32 floor, Dir dir
  ) const;

  const bool room_collision(i32 floor, const Room& test_room) const; 

  inline const Dir rand_dir() const { return static_cast<Dir>(rand() % 4); }

  Input& input_;
  Camera& camera_;
  Map& map_;
  Log& log_;

}; 

#endif
