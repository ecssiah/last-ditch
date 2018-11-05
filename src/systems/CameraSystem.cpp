#include <glm/glm.hpp>

#include "CameraSystem.h"
#include "../constants/CameraConstants.h"

CameraSystem::CameraSystem(Input& input, Render& render, Camera& camera) 
  : input_(input)
  , render_(render)
  , camera_(camera)
{
}

void CameraSystem::Initialize()
{
  camera_.speed = 2.5f;
  camera_.zoom = 1.0f;
  camera_.pos = {0.0f, 0.0f, 0.0f};
  camera_.x_dir = {-1.0f, 0.0f, 0.0f};
  camera_.y_dir = {0.0f, 1.0f, 0.0f};
  camera_.z_dir = {0.0f, 0.0f, -1.0f}; 
}

void CameraSystem::Update()
{
  auto inv_zoom {1.0f / camera_.zoom};
  auto modifier {inv_zoom * render_.dt * camera_.speed};

  if (input_.up) camera_.pos += modifier * camera_.y_dir; 
  if (input_.down) camera_.pos -= modifier * camera_.y_dir;
  if (input_.left) camera_.pos += modifier * camera_.x_dir; 
  if (input_.right) camera_.pos -= modifier * camera_.x_dir;

  if (input_.min) {
    camera_.zoom -= render_.dt;
    if (camera_.zoom < MIN_ZOOM) camera_.zoom = MIN_ZOOM; 
  }
  if (input_.mag) {
    camera_.zoom += render_.dt;
    if (camera_.zoom > MAX_ZOOM) camera_.zoom = MAX_ZOOM;
  }
}
