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
    msgs.push_back(
      "I am a long string waiting to be broken apart like a good monopoly would."
    );
    msgs.push_back("Wrapping and wrapping and warping and warping.");
    msgs.push_back("My name is Michael Chapman. This is a test, but only a test for the ages.");

  }

  bool changed;

  std::vector<std::string> msgs;

};

#endif
