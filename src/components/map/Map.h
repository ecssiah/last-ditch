#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/unordered_map.hpp>

#include "Floor.h"
#include "../../constants/MapConstants.h"

struct Map
{
  Map() 
    : floor_changed{false} 
    , cur_floor{0}
    , floors{NUM_FLOORS}
  { }

  bool floor_changed;
  unsigned cur_floor;

  std::vector<Floor> floors;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int version)
  {
    ar & cur_floor;
    ar & floors;
  }
};

#endif