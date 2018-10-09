#ifndef TEXTURE_MANAGER_H
#define TEXTURE_MANAGER_H

#include <SDL2/SDL.h>

class TextureManager 
{
public:
  static SDL_Renderer* renderer;

  static SDL_Texture* LoadTexture(const char* filename);
};

#endif // TEXTURE_MANAGER_H
