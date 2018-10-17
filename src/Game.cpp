#include <iostream>

#include "Game.h"

Game::Game()
  : input_system_(input_, render_)
  , time_system_(input_, render_)
  , render_system_(input_, render_, camera_, map_)
  , camera_system_(input_, render_, camera_)
  , map_system_(map_)
{
}

Game::~Game()
{
}

void Game::Initialize() 
{
  time_system_.Initialize();
  camera_system_.Initialize();
  render_system_.Initialize();
  input_system_.Initialize();
  map_system_.Initialize();

  while (!input_.exit)
  {
    time_system_.StartFrame();

    input_system_.Update();
    camera_system_.Update();
    map_system_.Update();
    render_system_.Update();

    time_system_.EndFrame();
  }
}
