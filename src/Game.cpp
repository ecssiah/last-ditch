#include <iostream>

#include "Game.h"

Game::Game() :
  time_system(input),
  render_system(input, window),
  input_system(input, window)
{
}

Game::~Game()
{
  std::cout << "Successfully Exited" << std::endl;
}

void Game::Initialize() 
{
  render_system.Initialize();
  time_system.Initialize();
  input_system.Initialize();

  for (double dt(0.0); !input.exit; time_system.Tick())
  {
    input_system.Update();
    render_system.Update(dt);
    dt = time_system.Update();
  }
}
