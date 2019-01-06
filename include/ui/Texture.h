#ifndef TEXTURE_H
#define TEXTURE_H

#include <SDL2/SDL.h>

#include "Element.h"

struct Texture : public Element
{
  Texture()
  {}

  SDL_Texture* texture;

};

#endif
