#ifndef LOG_H
#define LOG_H

#include <string>
#include <vector>

struct Log
{
  Log()
    : changed{true}
    , msgs{}
  {
    msgs.push_back("F");
    msgs.push_back("I am a long string waiting to broken apart like a good monopoly would.");
    msgs.push_back("1F");
    msgs.push_back(".C");

  }

  bool changed;

  std::vector<std::string> msgs;

};

#endif
