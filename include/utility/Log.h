#ifndef LOG_H
#define LOG_H

#include <string>
#include <vector>

struct Log
{
  Log()
    : changed{false}
    , msgs{}
  {
    msgs.push_back("Test message 1");
    msgs.push_back("Test message 2");

  }

  bool changed;

  std::vector<std::string> msgs;

};

#endif
