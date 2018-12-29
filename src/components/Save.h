#ifndef SAVE_H
#define SAVE_H

#include <string>
#include <boost/serialization/access.hpp>

#include "../Types.h"

struct Save
{
  Save()
    : filename{}
    , map_name{}
  {}

  std::string filename;
  std::string map_name;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const U32 version)
  {
    ar & filename;
    ar & map_name;
  }

};

#endif

