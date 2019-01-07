#ifndef UI_H
#define UI_H

#include <string>
#include <vector>
#include <unordered_map>

#include "Button.h"
#include "Element.h"
#include "Scalable.h"
#include "Scrollable.h"
#include "Scrollbar.h"
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
    , scrollbar_elements{}
    , texture_elements{}
  {}

  std::unordered_map<std::string, Text> text_elements;
  std::unordered_map<std::string, Scalable> scalable_elements;
  std::unordered_map<std::string, Window> window_elements;
  std::unordered_map<std::string, Button> button_elements;
  std::unordered_map<std::string, Scrollable> scrollable_elements;
  std::unordered_map<std::string, Scrollbar> scrollbar_elements;
  std::unordered_map<std::string, Texture> texture_elements;

};

#endif