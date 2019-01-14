#ifndef MAP_SYSTEM_H
#define MAP_SYSTEM_H

#include "Map.h"
#include "MapGenerator.h"
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
  void calculate_selected_tile();

  void clear_selection();
  void select_tile(i32 x, i32 y);

  Input& input_;
  Camera& camera_;
  Map& map_;
  Log& log_;

  MapGenerator map_generator_;
}; 

#endif
