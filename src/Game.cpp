#include <iostream>

#include "Game.h"

Game::Game()
  : time_system(input_)
  , render_system(input_, window_)
  , input_system(input_, window_)
{
}

Game::~Game()
{
}

void Game::Initialize() 
{
  render_system.Initialize();
  time_system.Initialize();
  input_system.Initialize();

  for (double dt(0.0); !input_.exit; time_system.Tick())
  {
    input_system.Update();
    render_system.Update(dt);
    dt = time_system.Update();
  }
}
