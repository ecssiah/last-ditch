#ifndef LOG_H
#define LOG_H

#include <string>
#include <vector>

struct Log
{
  Log()
    : changed{true}
    , msgs{}
  {}

  bool changed;

  std::vector<std::string> msgs;

};

#endif
