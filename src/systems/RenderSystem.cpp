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
  Input& input, Render& render, Camera& camera, Map& map
) 
  : input_(input)
  , render_(render)
  , camera_(camera)
  , map_(map)
{
}

RenderSystem::~RenderSystem()
{
  SDL_DestroyRenderer(renderer_);
  SDL_DestroyWindow(window_);
  SDL_Quit();

  cout << "Render System Shutdown" << endl;
}

void RenderSystem::Initialize()
{
  InitializeSDL();
  InitializeSDLImage();
  
  LoadTilesets();
}

void RenderSystem::Update()
{
  SDL_RenderClear(renderer_);
  SDL_RenderPresent(renderer_);
}

void RenderSystem::InitializeSDL()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    cout << "SDL_Init Error: " << SDL_GetError() << endl;
    return;
  }

  window_ = SDL_CreateWindow(
    "Last Ditch", 
    SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
    SCREEN_SIZE_X, SCREEN_SIZE_Y, 
    SDL_WINDOW_SHOWN
  );

  if (window_ == nullptr) {
    cout << "SDL_CreateWindow error: " << SDL_GetError() << endl;
    SDL_Quit();
    return;
  }

  renderer_ = SDL_CreateRenderer(
    window_, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC
  );

  if (renderer_ == nullptr){
    SDL_DestroyWindow(window_);
    cout << "SDL_CreateRenderer error: " << SDL_GetError() << endl;
    SDL_Quit();
    return;
  }
}

void RenderSystem::InitializeSDLImage()
{
  int img_flags {IMG_INIT_PNG};
  
  if (!(IMG_Init(img_flags) & img_flags)) {
    cout << "SDL_image error: " << IMG_GetError() << endl;
    return;
  }
}

void RenderSystem::LoadTilesets()
{
  map_tileset = LoadTexture("map_tileset"); 
  object_tileset = LoadTexture("object_tileset"); 
  character_tileset = LoadTexture("character_tileset"); 
}

SDL_Texture* RenderSystem::LoadTexture(string texturename)
{
  string filename {"assets/textures/" + texturename + ".png"};
  SDL_Surface* surface {IMG_Load(filename.c_str())};

  if (!surface) { 
    cout << "IMG_Load error: " << IMG_GetError() << endl;
  }

  SDL_Texture* texture {SDL_CreateTextureFromSurface(renderer_, surface)};

  if (!texture) {
    cout << "SDL_CreateTextureFromSurface error: " << SDL_GetError() << endl;
  }
  
  SDL_FreeSurface(surface);

  return texture;
}

