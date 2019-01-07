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
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");
    msgs.push_back("A test message for the ages...");

  }

  bool changed;

  std::vector<std::string> msgs;

};

#endif
