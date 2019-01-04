#ifndef LOG_SYSTEM_H
#define LOG_SYSTEM_H

#include <string>
#include <vector>

#include "../../include/utility/Log.h"

class LogSystem
{
public:
  LogSystem(Log& log);

private:
  Log& log_;

};

#endif