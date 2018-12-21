#include <iostream>
#include <string>
#include <vector>
#include <iterator>
#include <fstream>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"
#include "../constants/MapConstants.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map, Time& time
) 
  : input_{input}
  , render_{render}
  , camera_{camera}
  , map_{map}
  , ui_system_{input, render, map, time}
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


void RenderSystem::Initialize()
{
  InitializeSDL();
  InitializeSDLImage();
  
  LoadTilesets();

  ui_system_.Initialize(tilesets_["overlay"]);
}


void RenderSystem::Update()
{
  SDL_RenderClear(render_.renderer);

  RenderMap(); 
  ui_system_.Update();

  SDL_RenderPresent(render_.renderer);
}


void RenderSystem::RenderMap()
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
      RenderTile("floor", x, y);
      RenderTile("wall", x, y);
      RenderTile("object", x, y);
      RenderTile("entity", x, y);
      RenderTile("overlay", x, y);
    }
  }
}


void RenderSystem::RenderTile(string layer, int x, int y)
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
      render_.renderer, tilesets_[layer], 
      &tile.src, &dst, 
      tile.rotation, nullptr, tile.flip
    ); 
  }
}


void RenderSystem::InitializeSDL()
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
    cout << "SDL_CreateRenderer error: " << SDL_GetError() << endl;
    SDL_Quit();
    return;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "0");
}


void RenderSystem::InitializeSDLImage()
{
  int img_flags{IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    cout << "SDL_image error: " << IMG_GetError() << endl;
    return;
  }
}


void RenderSystem::LoadTilesets()
{
  tilesets_["floor"] = LoadTexture("map_tileset"); 
  tilesets_["wall"] = tilesets_["floor"];
  tilesets_["object"] = LoadTexture("object_tileset"); 
  tilesets_["entity"] = LoadTexture("entity_tileset"); 
  tilesets_["overlay"] = LoadTexture("overlay_tileset");
}


SDL_Texture* RenderSystem::LoadTexture(string texturename)
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

