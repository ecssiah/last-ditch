#ifndef TEXT_H
#define TEXT_H

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "Element.h"
#include "../utility/Types.h"

struct Text : public Element
{
  Text()
    : size{14}
    , content{}
    , font{}
    , texture{}
    , color{255, 255, 255}
  {}

  i32 size;

  std::string content;
  std::string font;
  std::string texture;

  SDL_Color color;

};

#endif
