#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "../interface/Input.h"
#include "Map.h"
#include "../render/Camera.h"
#include "../utility/Log.h"
#include "MapGenerator.h"

class MapSystem
{
public:
  MapSystem(Input& input, Map& map, Camera& camera, Log& log);

  void init();
  void update();

private:
  void calculate_selected_tile();

  void clear_selection();
  void select_tile(i32 x, i32 y);

  Input& input_;
  Map& map_;
  Camera& camera_;
  Log& log_;

  MapGenerator map_generator_;
}; 

#endif
