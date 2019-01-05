#ifndef LOGGING_H
#define LOGGING_H

#include <string>
#include <iostream>

#include "Types.h"

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

#endif