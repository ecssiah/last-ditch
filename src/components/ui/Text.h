#ifndef TEXT_H
#define TEXT_H

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "Element.h"
#include "../../Types.h"

struct Text : public Element
{
  Text()
    : size{14}
    , text{}
    , color{255, 255, 255}
    , font{nullptr}
    , texture{nullptr}
  {}

  I32 size;

  std::string text;

  SDL_Color color;

  TTF_Font* font;
  SDL_Texture* texture;

};

#endif
