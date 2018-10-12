#include <iostream>

#include "Game.h"

Game::Game()
  : time_system(input_)
  , input_system(input_, window_)
  , render_system(input_, window_)
{
}

Game::~Game()
{
}

void Game::Initialize() 
{
  time_system.Initialize();
  input_system.Initialize();
  render_system.Initialize();

  while (!input_.exit)
  {
    auto dt {time_system.Update()};

    input_system.Update();
    render_system.Update(dt);

    time_system.Tick();
  }
}
