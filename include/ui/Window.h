#ifndef WINDOW_H
#define WINDOW_H

#include <SDL2/SDL.h>

#include "Scalable.h"
#include "../utility/Types.h"
#include "../map/MapConstants.h"

struct Window: public Element
{
  Window()
    : Window("window1")
  {}

  Window(const std::string& _type)
    : type{_type}
    , base{_type}
  {}

  Scalable base;

  std::string type;

};

#endif
