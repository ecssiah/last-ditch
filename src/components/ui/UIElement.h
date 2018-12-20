#ifndef UI_ELEMENT_H
#define UI_ELEMENT_H

#include <SDL2/SDL.h>

struct UIElement
{
  UIElement()
    : rect{}
  {}

  SDL_Rect rect;

};

#endif // UI_ELEMENT_H
