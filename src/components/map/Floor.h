#ifndef FLOOR_H
#define FLOOR_H

#include <string>
#include <unordered_map>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "Layer.h"

struct Floor
{
  Floor()
  {}

  std::unordered_map<std::string, Layer> layers;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int version)
  {
    ar & layers;
  }
};

#endif // FLOOR_H
