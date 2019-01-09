#ifndef TEXT_H
#define TEXT_H

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "Element.h"
#include "UIConstants.h"
#include "../utility/Types.h"

struct Text : public Element
{
  Text()
    : size{14}
    , content{}
    , font{}
    , texture{}
    , align{LEFT_ALIGN}
    , color{255, 255, 255}
  {}

  i32 size;

  std::string content;
  std::string font;
  std::string texture;

  Align align;

  SDL_Color color;

};

#endif
