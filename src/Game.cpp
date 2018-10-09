#include "Game.h"
#include "TextureManager.h"

Game::Game()
{
}

Game::~Game()
{
}

void Game::Init(
  const char* title, 
  int xpos, int ypos, int width, int height, 
  bool fullscreen
) {

  int flags = 0;
  if (fullscreen)
  {
    flags = SDL_WINDOW_FULLSCREEN;
  }

  if (SDL_Init(SDL_INIT_EVERYTHING) == 0)
  {
    window = SDL_CreateWindow(title, xpos, ypos, width, height, flags);
    renderer = SDL_CreateRenderer(window, -1, 0);
    if (renderer)
    {
      SDL_SetRenderDrawColor(renderer, 40, 0, 40, SDL_ALPHA_OPAQUE);
    }
    is_running = true;
  }

  player = new GameObject("assets/textures/character1.png", renderer, 0, 0);
}

void Game::HandleEvents()
{
  SDL_Event event;
  SDL_PollEvent(&event);

  switch (event.type) {
  case SDL_QUIT:
    is_running = false;
    break;
  default:
    break;
  }
}

void Game::Update()
{
  player->Update();
}

void Game::Render()
{
  SDL_RenderClear(renderer);
  player->Render();
  SDL_RenderPresent(renderer);
}

void Game::Clean()
{
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);
  SDL_Quit();

  std::cout << "SDL Quit" << std::endl;
}
