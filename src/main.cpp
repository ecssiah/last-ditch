#include "Game.h"

Game* game = nullptr;

int main(int argc, char *argv[])
{
  const int FPS = 60;
  const int frame_delay = 1000 / FPS;

  Uint32 frame_start;
  int frame_time;

  game = new Game();

  game->Init(
    "Last Ditch",
    SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
    800, 600, false
  );

  while (game->IsRunning()) 
  {
    frame_start = SDL_GetTicks();

    game->HandleEvents();
    game->Update();
    game->Render();

    frame_time = SDL_GetTicks() - frame_start;

    if (frame_delay > frame_time)
    {
      SDL_Delay(frame_delay - frame_time);
    }
  }

  game->Clean();

  return 0;
}
