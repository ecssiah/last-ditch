#ifndef TEXT_ELEMENT_H
#define TEXT_ELEMENT_H

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "UIElement.h"

struct TextElement : public UIElement
{
  TextElement()
    : size{14}
    , color{255, 255, 255}
  {}

  unsigned size;

  std::string text;

  SDL_Color color;

  TTF_Font* font;
  SDL_Texture* texture;

};

#endif // UI_ELEMENT_H
