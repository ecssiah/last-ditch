#ifndef RENDER_H
#define RENDER_H

#include <string>
#include <unordered_map>
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>

#include "../utility/Types.h"
#include "../constants/MapConstants.h"
#include "../constants/RenderConstants.h"
#include "../ui/Text.h"
#include "../ui/Scalable.h"
#include "../ui/Window.h"
#include "../ui/Button.h"
#include "../ui/Scrollable.h"

struct Render
{
  Render() 
    : dt{FRAME_TIME} 
    , window{nullptr}
    , renderer{nullptr}
    , fonts{}
    , textures{}
    , scale{}
    , grid{false}
    , grid_src{2 * TILE_SIZE, 2 * TILE_SIZE, TILE_SIZE, TILE_SIZE}
    , grid_dst{0 * TILE_SIZE, 0 * TILE_SIZE, TILE_SIZE, TILE_SIZE}
  {
  }

  f32 dt;

  SDL_Window* window;
  SDL_Renderer* renderer;

  std::unordered_map<std::string, TTF_Font*> fonts;
  std::unordered_map<std::string, SDL_Texture*> textures;

  f32 scale;

  bool grid;

  SDL_Rect grid_src;
  SDL_Rect grid_dst;

};

#endif
