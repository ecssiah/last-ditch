#include <iostream>
#include <string>
#include <vector>
#include <fstream>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"
#include "../constants/MapConstants.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map
) 
  : input_{input}
  , render_{render}
  , camera_{camera}
  , map_{map}
{
}


RenderSystem::~RenderSystem()
{
  IMG_Quit();

  SDL_DestroyRenderer(render_.renderer);
  SDL_DestroyWindow(render_.window);
  SDL_Quit();

  cout << "RenderSystem shutdown" << endl;
}


void RenderSystem::init()
{
  init_SDL();
  init_SDL_image();
  
  load_tilesets();
}


void RenderSystem::update()
{
  SDL_RenderClear(render_.renderer);

  render_map(); 
}


void RenderSystem::display()
{
  SDL_RenderPresent(render_.renderer);
}


void RenderSystem::render_map()
{
  int x_min(camera_.pos.x - VIEW_X * 1.0f / camera_.zoom); 
  int y_min(camera_.pos.y - VIEW_Y * 1.0f / camera_.zoom);
  int x_max(camera_.pos.x + VIEW_X * 1.0f / camera_.zoom);
  int y_max(camera_.pos.y + VIEW_Y * 1.0f / camera_.zoom); 
  
  if (x_min < 0) x_min = 0;
  if (y_min < 0) y_min = 0;
  if (x_max > TILES_PER_LAYER - 1) x_max = TILES_PER_LAYER - 1;
  if (y_max > TILES_PER_LAYER - 1) y_max = TILES_PER_LAYER - 1;

  for (auto x{x_min}; x <= x_max; ++x) { 
    for (auto y{y_min}; y <= y_max; ++y) {
      render_tile("floor", x, y);
      render_tile("wall", x, y);
      render_tile("object", x, y);
      render_tile("entity", x, y);
      render_tile("overlay", x, y);
    }
  }
}


void RenderSystem::render_tile(const string& layer, int x, int y)
{
  Tile& tile{map_.floors[map_.cur_floor].layers[layer].tiles[x][y]};

  if (tile.active) {
    float scale_factor{camera_.zoom * TILE_SIZE};

    SDL_Rect dst;
    dst.x = (x - camera_.pos.x) * scale_factor + HALF_SCREEN_SIZE_X; 
    dst.y = (y - camera_.pos.y) * scale_factor + HALF_SCREEN_SIZE_Y;
    dst.w = scale_factor + 2;
    dst.h = scale_factor + 2;

    SDL_RenderCopyEx(
      render_.renderer, render_.textures[layer], 
      &tile.src, &dst, tile.rotation, nullptr, tile.flip
    ); 
  }
}


void RenderSystem::init_SDL()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    cout << "SDL_Init Error: " << SDL_GetError() << endl;
    return;
  }

  render_.window = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    SDL_WINDOW_SHOWN
  );

  if (render_.window == nullptr) {
    cout << "SDL_CreateWindow error: " << SDL_GetError() << endl;
    SDL_Quit();
    return;
  }

  render_.renderer = SDL_CreateRenderer(
    render_.window, -1, 
    SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (render_.renderer == nullptr){
    SDL_DestroyWindow(render_.window);
    SDL_Quit();

    cout << "SDL_CreateRenderer error: " << SDL_GetError() << endl;
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "0");
}


void RenderSystem::init_SDL_image()
{
  int img_flags{IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    cout << "SDL_image error: " << IMG_GetError() << endl;
    return;
  }
}


void RenderSystem::load_tilesets()
{
  render_.textures["floor"] = load_texture("map_tileset"); 
  render_.textures["wall"] = render_.textures["floor"];
  render_.textures["object"] = load_texture("object_tileset"); 
  render_.textures["entity"] = load_texture("entity_tileset"); 
  render_.textures["overlay"] = load_texture("overlay_tileset");
}


SDL_Texture* RenderSystem::load_texture(const string& texturename)
{
  string filename{"assets/textures/" + texturename + ".png"};
  SDL_Surface* surface{IMG_Load(filename.c_str())};

  if (!surface) { 
    cout << "IMG_Load error: " << IMG_GetError() << endl;
    return nullptr;
  }

  SDL_Texture* texture{SDL_CreateTextureFromSurface(render_.renderer, surface)};

  if (!texture) {
    cout << "SDL_CreateTextureFromSurface error: " << SDL_GetError() << endl;
    return nullptr;
  }
  
  SDL_FreeSurface(surface);

  return texture;
}

