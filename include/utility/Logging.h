#ifndef LOGGING_H
#define LOGGING_H

#include <string>
#include <iostream>
#include <sstream>
#include <SDL2/SDL.h>

#include "Types.h"
#include "Log.h"

namespace 
{
  inline void msg(Log& _log, const std::string& msg) 
  {
    _log.changed = true;
    _log.msgs.insert(_log.msgs.begin(), msg );
  }

  inline void print(SDL_Rect& rect) {
    std::cout << "<";
    std::cout << rect.x << " " << rect.y << " " << rect.w << " " << rect.h; 
    std::cout << ">" << std::endl;
  }
}

#endif