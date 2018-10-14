#include <iostream>

#include "Game.h"

Game::Game()
  : input_system(input_, window_)
  , time_system(input_, window_)
  , render_system(input_, window_, camera_)
  , camera_system(input_, window_, camera_)
{
}

Game::~Game()
{
}

void Game::Initialize() 
{
  time_system.Initialize();
  input_system.Initialize();
  camera_system.Initialize();
  render_system.Initialize();

  while (!input_.exit)
  {
    time_system.StartFrame();

    input_system.Update();
    camera_system.Update();
    render_system.Update();

    time_system.EndFrame();
  }
}
