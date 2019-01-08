#ifndef LOGGING_H
#define LOGGING_H

#include <string>
#include <iostream>
#include <sstream>

#include "Types.h"
#include "Log.h"

namespace {
  inline void mlog(const std::string& msg, i32 level = 0)
  {
    for (auto i{0}; i < level; i++) std::cout << " ";
    std::cout << msg << std::endl;
  }

  inline void elog(const std::string& msg, i32 level = 0)
  {
    for (auto i{0}; i < level; i++) std::cerr << " ";
    std::cerr << msg << std::endl;
  }

  inline void ulog(Log& _log, const std::string& msg) 
  {
    _log.changed = true;
    _log.msgs.insert(_log.msgs.begin(), msg );

    while (_log.msgs.size() > 20) _log.msgs.erase(_log.msgs.end() - 1);
  }
}

#endif