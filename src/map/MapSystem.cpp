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

  i32 targetx = floor(screenx + camera_.pos.x);
  i32 targety = floor(screeny + camera_.pos.y);

  auto xcheck{targetx < 0 || targety > TILES_PER_LAYER - 1};
  auto ycheck{targety < 0 || targety > TILES_PER_LAYER - 1}; 

  if (xcheck || ycheck) {
    input_.selectx = -1;
    input_.selecty = -1;
    input_.selectfloor = -1;

    ::msg(log_, "Selected: out of bounds");
  } else {
    clear_selection();

    input_.selectx = targetx;
    input_.selecty = targety;
    input_.selectfloor = map_.cur_floor;

    select_tile(input_.selectx, input_.selecty, input_.selectfloor);

    string msg{
      "Selected: " + to_string(input_.selectx) + ", " + to_string(input_.selecty)
    }; 
    ::msg(log_, msg);
  }
}


void MapSystem::select_tile(i32 x, i32 y, i32 floor)
{
  map_generator_.set_tile("overlay", x, y, floor, "select");
}


void MapSystem::clear_selection() {
  if (input_.selectx != -1) {
    if (map_generator_.grid_active()) {
      map_generator_.set_tile(
        "overlay", input_.selectx, input_.selecty, input_.selectfloor, "grid"
      );
    } else {
      map_generator_.set_active(
        "overlay", input_.selectx, input_.selecty, input_.selectfloor, false
      );
    }
  }
}

