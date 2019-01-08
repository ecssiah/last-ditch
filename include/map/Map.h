#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/unordered_map.hpp>

#include "Floor.h"
#include "MapConstants.h"
#include "../utility/Types.h"

struct Map
{
  Map() 
    : floor_changed{false} 
    , cur_floor{0}
    , floors{(u32)NUM_FLOORS}
    , selected{}
  { }

  bool floor_changed;
  u16 cur_floor;

  std::vector<Floor> floors;
  std::vector<SDL_Point> selected;

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
