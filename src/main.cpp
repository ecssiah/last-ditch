#include "Game.h"

Game* game = nullptr;

int main(int argc, char *argv[])
{
  game = new Game();

  game->init(
    "Last Ditch",
    SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
    800, 600, false
  );

  while (game->running()) {
    game->handle_events();
    game->update();
    game->render();
  }

  game->clean();

  return 0;
}
