#include <iostream>
#include <glm/glm.hpp>

#include "../../include/render/CameraConstants.h"
#include "../../include/render/CameraSystem.h"
#include "../../include/map/MapConstants.h"

using namespace std;

CameraSystem::CameraSystem(Input& input, Render& render, Camera& camera) 
  : input_{input}
  , render_{render}
  , camera_{camera}
{
}

void CameraSystem::init()
{
}

void CameraSystem::update()
{
  if (input_.mag) camera_.inc_zoom();
  if (input_.min) camera_.dec_zoom();

  if (input_.right) camera_.move(render_.dt, RIGHT);
  if (input_.up)    camera_.move(render_.dt, UP);
  if (input_.left)  camera_.move(render_.dt, LEFT);
  if (input_.down)  camera_.move(render_.dt, DOWN);
}