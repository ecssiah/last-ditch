#ifndef WINDOW_ELEMENT_H
#define WINDOW_ELEMENT_H

#include <SDL2/SDL.h>

struct WindowElement
{
  WindowElement()
    : texture{nullptr}
  {}

  SDL_Texture* texture;
};

#endif // WINDOW_ELEMENT_H
