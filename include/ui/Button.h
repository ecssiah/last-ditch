#ifndef BUTTON_H
#define BUTTON_H

#include <iostream>
#include <SDL2/SDL.h>

#include "Scalable.h"
#include "../Types.h"
#include "../map/MapConstants.h"

struct Button : public Element
{
  Button()
    : Button{"button1"}
  {}

  Button(const std::string& _type)
    : type{_type}
    , text{}
    , active{false}
    , base{_type + "-off"}
    , pressed{_type + "-on"}
  {
  }

  Scalable base;
  Scalable pressed;

  std::string type;
  std::string text;

  bool active;

};

#endif
