#ifndef USER_H
#define USER_H

#include <string>
#include <vector>

#include "Save.h"

struct User
{
  User()
    : username{}
    , saves{}
  {}

  std::string username;

  std::vector<Save> saves;
};

#endif

