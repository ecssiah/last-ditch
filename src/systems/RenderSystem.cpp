#include "RenderSystem.h"
#include "../constants/RenderConstants.h"

RenderSystem::RenderSystem() :
  window(nullptr),
  renderer(nullptr)
{
}

RenderSystem::~RenderSystem()
{
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);
  SDL_Quit();
}

void RenderSystem::Initialize()
{
  if (SDL_Init(SDL_INIT_EVERYTHING) == 0)
  {
    window = SDL_CreateWindow(
      "Last Ditch", 
      SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 
      Render::SCREEN_SIZE_X, Render::SCREEN_SIZE_Y, 
      0
    );

    renderer = SDL_CreateRenderer(window, -1, 0);
    if (renderer)
    {
      SDL_SetRenderDrawColor(renderer, 40, 0, 40, SDL_ALPHA_OPAQUE);
    }
  }
}

void RenderSystem::Update(const double& dt)
{
  SDL_RenderClear(renderer);

  // render stuff

  SDL_RenderPresent(renderer);
}
