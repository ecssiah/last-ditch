#include "../../include/map/MapSystem.h"

#include <iostream>

#include "../../include/utility/Logging.h"
#include "../../include/constants/MapConstants.h"
#include "../../include/constants/RenderConstants.h"

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
  }

  if (input_.ascend && map_.cur_floor < NUM_FLOORS) {
    map_.floor_changed = true;
    map_.cur_floor++; 
  }
}


void MapSystem::calculate_selected_tile()
{
  f32 screenx{(input_.mx - HALF_SCREEN_SIZE_X) / (f32)TILE_SIZE / camera_.zoom};
  f32 screeny{(input_.my - HALF_SCREEN_SIZE_Y) / (f32)TILE_SIZE / camera_.zoom};

  i32 targetx{(i32)floor(screenx + camera_.pos.x)};
  i32 targety{(i32)floor(screeny + camera_.pos.y)};

  if (select_tile(targetx, targety)) {
    string msg{"Selected: ["};
    msg += to_string(input_.selectx) + "," + to_string(input_.selecty) + ",";
    msg += to_string(input_.selectfloor) + "]"; 

    ::msg(log_, msg);
  } else {
    ::msg(log_, "Selected: [invalid]");
  }
}


bool MapSystem::select_tile(i32 x, i32 y)
{
  const auto x_in_bounds{x >= 0 && x <= TILES_PER_LAYER - 1};
  const auto y_in_bounds{y >= 0 && y <= TILES_PER_LAYER - 1}; 

  if (x_in_bounds && y_in_bounds) {
    clear_selection();

    input_.selectx = x;
    input_.selecty = y;
    input_.selectfloor = map_.cur_floor;

    map_generator_.set_tile("overlay", x, y, map_.cur_floor, "select");

    return true;
  } else {
    return false;
  }
}


void MapSystem::clear_selection() {
  if (input_.selectx == -1 || input_.selecty == -1) return;

  map_generator_.set_active(
    "overlay", input_.selectx, input_.selecty, input_.selectfloor, false
  );
}

