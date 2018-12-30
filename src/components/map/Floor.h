#ifndef FLOOR_H
#define FLOOR_H

#include <string>
#include <unordered_map>
#include <boost/serialization/access.hpp>

#include "Layer.h"
#include "../../Types.h"

struct Floor
{
  Floor()
    : layers{}
  {}

  std::unordered_map<std::string, Layer> layers;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & layers;
  }
};

#endif
