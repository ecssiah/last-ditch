#include "../../include/map/MapSystem.h"

#include <iostream>

#include "../../include/utility/Logging.h"
#include "../../include/map/MapConstants.h"
#include "../../include/render/RenderConstants.h"

using namespace std;

MapSystem::MapSystem(Input& input, Camera& camera, Map& map, Log& log)
  : input_{input}
  , camera_{camera}
  , map_{map}
  , log_{log}
  , map_generator_{map}
{
}


void MapSystem::init()
{
  cout << "MapSystem initializing" << endl;

  map_generator_.generate_map();
}


void MapSystem::update()
{
  if (input_.lclick) calculate_selected_tile();

  if (map_.floor_changed) map_.floor_changed = false;

  if (input_.descend && map_.cur_floor > 1) {
    map_.floor_changed = true;

    map_.cur_floor--;
    map_.section = map_generator_.get_section_name(map_.cur_floor);
  }

  if (input_.ascend && map_.cur_floor < NUM_FLOORS) {
    map_.floor_changed = true;

    map_.cur_floor++; 
    map_.section = map_generator_.get_section_name(map_.cur_floor);
  }
}


void MapSystem::calculate_selected_tile()
{
  f32 tx{(input_.mx - HALF_SCREEN_SIZE_X) / (f32)TILE_SIZE / camera_.zoom};
  f32 ty{(input_.my - HALF_SCREEN_SIZE_Y) / (f32)TILE_SIZE / camera_.zoom};

  input_.sx = floor(tx + camera_.pos.x);
  input_.sy = floor(ty + camera_.pos.y);

  auto xcheck{input_.sx < 0 || input_.sx > TILES_PER_LAYER - 1};
  auto ycheck{input_.sy < 0 || input_.sy > TILES_PER_LAYER - 1}; 

  if (xcheck || ycheck) {
    input_.sx = -1;
    input_.sy = -1;
    ::msg(log_, "Selected: invalid");
  } else {
    clear_selection();
    select_tile(input_.sx, input_.sy);
    ::msg(
      log_, 
      "Selected: " + to_string(input_.sx) + ", " + to_string(input_.sy)
    );
  }
}


void MapSystem::select_tile(i32 x, i32 y)
{
  map_generator_.set_tile("overlay", input_.sx, input_.sy, 1, "selection");
}


void MapSystem::clear_selection()
{
  for (auto x{0}; x < TILES_PER_LAYER; x++)
    for (auto y{0}; y < TILES_PER_LAYER; y++)
      map_generator_.set_active("overlay", x, y, 1, false);
}

