#ifndef UI_H
#define UI_H

#include <string>
#include <vector>
#include <unordered_map>

#include "Button.h"
#include "Element.h"
#include "Scalable.h"
#include "Scrollable.h"
#include "Texture.h"
#include "Text.h"
#include "Window.h"

struct UI
{
  UI()
    : text_elements{}
    , scalable_elements{}
    , window_elements{}
    , button_elements{}
    , scrollable_elements{}
    , texture_elements{}
    , menu_elements{}
    , hud_elements{}
  {}

  std::unordered_map<std::string, Text> text_elements;
  std::unordered_map<std::string, Scalable> scalable_elements;
  std::unordered_map<std::string, Window> window_elements;
  std::unordered_map<std::string, Button> button_elements;
  std::unordered_map<std::string, Scrollable> scrollable_elements;
  std::unordered_map<std::string, Texture> texture_elements;

  std::vector<Element*> menu_elements;
  std::vector<Element*> hud_elements;

};

#endif