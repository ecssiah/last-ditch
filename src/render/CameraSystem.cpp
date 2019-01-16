#include <iostream>
#include <glm/glm.hpp>

#include "../../include/constants/CameraConstants.h"
#include "../../include/constants/MapConstants.h"
#include "../../include/render/CameraSystem.h"

using namespace std;

CameraSystem::CameraSystem(Input& input, Render& render, Camera& camera) 
  : input_{input}
  , render_{render}
  , camera_{camera}
{
}


void CameraSystem::init()
{
  render_.scale = camera_.zoom * TILE_SIZE;
}


void CameraSystem::update()
{
  if (input_.mag) inc_zoom();
  if (input_.min) dec_zoom();

  if (input_.up)    move(UP);
  if (input_.down)  move(DOWN);
  if (input_.left)  move(LEFT);
  if (input_.right) move(RIGHT);
}


void CameraSystem::move(Dir dir)
{
  auto ds{camera_.speed * render_.dt * camera_.inv_zoom};

  switch (dir) {
  case UP:    camera_.pos -= ds * camera_.ydir; break;
  case DOWN:  camera_.pos += ds * camera_.ydir; break;
  case LEFT:  camera_.pos -= ds * camera_.xdir; break;
  case RIGHT: camera_.pos += ds * camera_.xdir; break;
  };
}


void CameraSystem::inc_zoom() 
{ 
  camera_.zoom = min(MAX_ZOOM, camera_.zoom * 2); 
  camera_.inv_zoom = 1.0 / camera_.zoom;

  render_.scale = camera_.zoom * TILE_SIZE;
  render_.grid_dst.w = render_.scale;
  render_.grid_dst.h = render_.scale;
}


void CameraSystem::dec_zoom() 
{ 
  camera_.zoom = max(MIN_ZOOM, camera_.zoom / 2); 
  camera_.inv_zoom = 1.0 / camera_.zoom;

  render_.scale = camera_.zoom * TILE_SIZE;
  render_.grid_dst.w = render_.scale;
  render_.grid_dst.h = render_.scale;
}