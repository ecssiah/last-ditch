#include <iostream>

#include "Game.h"

Game::Game() :
  render_system(input),
  time_system(input),
  input_system(input)
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
  input_system.Initialize(render_system.GetWindow());

  for (double dt(0.0); !input.exit; time_system.Tick())
  {
    input_system.Update();
    render_system.Update(dt);
    dt = time_system.Update();
  }
}
