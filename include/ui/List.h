#ifndef LIST_H
#define LIST_H

#include <string>

#include "Element.h"

struct List : public Element
{
  List()
    : font{} 
    , texture{}
  {}

  std::string font;
  std::string texture;
  // SDL_Texture* texture;

};

#endif
