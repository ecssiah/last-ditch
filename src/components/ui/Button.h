#ifndef BUTTON_H
#define BUTTON_H

#include "Element.h"

struct Button : public Element
{
  Button()
    text{}
  {}

  std::string text;

};

#endif
