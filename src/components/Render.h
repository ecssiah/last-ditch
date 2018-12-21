#ifndef RENDER_H
#define RENDER_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>

struct Render
{
  Render() 
    : dt{} 
    , window{nullptr}
    , renderer{nullptr}
    , textures{}
  {}

  float dt;

  SDL_Window* window;
  SDL_Renderer* renderer;

  std::unordered_map<std::string, SDL_Texture*> textures;

};

#endif
