#ifndef BUTTON_H
#define BUTTON_H

#include <iostream>
#include <SDL2/SDL.h>

#include "Scalable.h"
#include "Text.h"
#include "../utility/Types.h"
#include "../map/MapConstants.h"

struct Button : public Element
{
  Button()
    : type{}
    , active{false}
    , base{}
    , pressed{}
    , label{}
  {
  }

  bool active;

  std::string type;

  Scalable base;
  Scalable pressed;
  Text label;

};

#endif
