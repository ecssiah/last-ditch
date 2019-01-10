#ifndef LIST_H
#define LIST_H

#include <string>
#include <vector>

#include "Element.h"

struct List : public Element
{
  List()
    : font{} 
    , texture{}
    , items{}
  {}

  std::string font;
  std::string texture;

  std::vector<std::string> items;

};

#endif
