#include <iostream>
#include <string>
#include <vector>
#include <iterator>
#include <fstream>
#include <functional>

#include <SDL2/SDL.h>
#include <glm/glm.hpp>

#include "RenderSystem.h"
#include "../constants/RenderConstants.h"
#include "../constants/MapConstants.h"

using namespace std;

RenderSystem::RenderSystem(
  Input& input, Render& render, Camera& camera, Map& map
) 
  : input_(input)
  , render_(render)
  , camera_(camera)
  , map_(map)
{
}

RenderSystem::~RenderSystem()
{
  cout << "Render System Shutdown" << endl;
}

void RenderSystem::Initialize()
{
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    cout << "SDL_Init Error: " << SDL_GetError() << endl;
    return;
  }
}

void RenderSystem::Update()
{
}

