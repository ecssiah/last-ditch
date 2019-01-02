#ifndef BUTTON_H
#define BUTTON_H

#include <iostream>
#include <SDL2/SDL.h>

#include "Scalable.h"
#include "../utility/Types.h"
#include "../map/MapConstants.h"

struct Button : public Element
{
  Button()
    : type{}
    , text{}
    , active{false}
    , base{}
    , pressed{}
  {
  }

  bool active;

  std::string type;
  std::string text;

  Scalable base;
  Scalable pressed;

};

#endif
