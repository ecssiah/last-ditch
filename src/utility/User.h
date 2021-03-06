#ifndef USER_H
#define USER_H

#include <string>
#include <vector>
#include <boost/serialization/access.hpp>
#include <boost/serialization/vector.hpp>

#include "Types.h"
#include "Save.h"

struct User
{
  User()
    : username{}
    , saves{}
  {}

  std::string username;

  std::vector<Save> saves;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
  {
    ar & username;
    ar & saves;
  }

};

#endif

