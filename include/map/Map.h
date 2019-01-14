#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/unordered_map.hpp>

#include "Floor.h"
#include "TileInfo.h"
#include "../utility/Types.h"
#include "../constants/MapConstants.h"

struct Map
{
  Map() 
    : floor_changed{false} 
    , cur_floor{1}
    , floors{(u32)NUM_FLOORS + 1}
    , selected{}
    , tile_data{}
  { }

  bool floor_changed;
  u16 cur_floor;

  std::vector<Floor> floors;
  std::vector<SDL_Point> selected;

  std::unordered_map<std::string, TileInfo> tile_data;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & cur_floor;
    ar & floors;
  }
};

#endif
