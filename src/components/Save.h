#ifndef SAVE_H
#define SAVE_H

#include <string>

struct Save
{
  Save()
    : filename{}
    , map_name{}
  {}

  std::string filename;
  std::string map_name;

};

#endif

