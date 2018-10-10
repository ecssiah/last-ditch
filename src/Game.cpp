#include <iostream>

#include "Game.h"

Game::Game() :
  time_system(input),
  input_system(input),
  render_system(input)
{
}

Game::~Game()
{
  std::cout << "Successfully Exited" << std::endl;
}

void Game::Initialize() 
{
  time_system.Initialize();
  input_system.Initialize();
  render_system.Initialize();

  for (double dt(0.0); !input.exit; time_system.Tick())
  {
    input_system.Update();
    render_system.Update(dt);
    dt = time_system.Update();
  }
}
