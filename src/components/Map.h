#ifndef MAP_H
#define MAP_H

#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>
#include <boost/serialization/vector.hpp>
#include <boost/serialization/unordered_map.hpp>

#include "Floor.h"

struct Map
{
  Map() 
    : cur_floor(0)
    , floors()
  { }

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

#endif // MAP_H
