#ifndef WINDOW_H
#define WINDOW_H

#include <SDL2/SDL.h>

#include "Scalable.h"
#include "../utility/Types.h"
#include "../constants/MapConstants.h"

struct Window: public Element
{
  Window()
    : type{}
    , base{}
  {}

  Scalable base;

  std::string type;

};

#endif
