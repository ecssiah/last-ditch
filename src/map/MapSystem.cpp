#include <iostream>

#include "../../include/map/MapSystem.h"
#include "../../include/map/MapConstants.h"

using namespace std;

MapSystem::MapSystem(Input& input, Map& map)
  : input_{input}
  , map_{map}
  , map_generator_{map}
{
}


void MapSystem::init()
{
  map_generator_.generate_map();

  cout << "MapSystem init" << endl;
}


void MapSystem::update()
{
  if (map_.floor_changed) map_.floor_changed = false;

  if (input_.descend && map_.cur_floor > 0) {
    map_.cur_floor--;
    map_.floor_changed = true;
  }
  if (input_.ascend && map_.cur_floor < NUM_FLOORS - 1) {
    map_.cur_floor++; 
    map_.floor_changed = true;
  }
}

