#include <SDL2/SDL_image.h>

#include "TextureManager.h"

SDL_Renderer* TextureManager::renderer = nullptr;

SDL_Texture* TextureManager::LoadTexture(const char* filename) {

  SDL_Surface* temp_surface = IMG_Load(filename);
  SDL_Texture* texture = SDL_CreateTextureFromSurface(
    Game::renderer, temp_surface
  );
  SDL_FreeSurface(temp_surface);
  
  return texture;
}
