#ifndef RENDER_H
#define RENDER_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>

#include "../constants/SimulationConstants.h"

struct Render
{
  Render() 
    : dt{FRAME_TIME} 
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
