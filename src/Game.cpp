#include "Game.h"

SDL_Texture* player_texture;
SDL_Rect src_rect, dest_rect;

Game::Game()
{
}

Game::~Game()
{
}

void Game::init(
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

  SDL_Surface* temp_surface = IMG_Load("resources/textures/character1.png");
  player_texture = SDL_CreateTextureFromSurface(renderer, temp_surface);
  SDL_FreeSurface(temp_surface);
}

void Game::handle_events()
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

void Game::update()
{
  dest_rect.w = 64;  
  dest_rect.h = 64;
}

void Game::render()
{
  SDL_RenderClear(renderer);
  SDL_RenderCopy(renderer, player_texture, NULL, &dest_rect);
  SDL_RenderPresent(renderer);
}

void Game::clean()
{
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);
  SDL_Quit();

  std::cout << "SDL Quit" << std::endl;
}
