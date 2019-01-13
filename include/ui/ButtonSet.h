#ifndef BUTTON_SET_H
#define BUTTON_SET_H

#include <string>
#include <unordered_map>

#include "../utility/Types.h"
#include "Element.h"
#include "Button.h"

struct ButtonSet : public Element
{
  ButtonSet()
    : buttons{}
  {
  }

  std::unordered_map<std::string, Button> buttons;

};

#endif
