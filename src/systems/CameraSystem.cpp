#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "CameraSystem.h"

CameraSystem::CameraSystem(Input& input, Window& window, Camera& camera) 
  : input_(input)
  , window_(window)
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
  auto modifier {inv_zoom * window_.dt * camera_.speed};

  if (input_.up) camera_.pos += modifier * camera_.y_dir; 
  if (input_.down) camera_.pos -= modifier * camera_.y_dir;
  if (input_.left) camera_.pos += modifier * camera_.x_dir; 
  if (input_.right) camera_.pos -= modifier * camera_.x_dir;
  if (input_.min) 
  {
    camera_.zoom -= window_.dt;
    if (camera_.zoom < 0.5f) camera_.zoom = 0.5f; 
  }
  if (input_.mag)
  {
    camera_.zoom += window_.dt;
    if (camera_.zoom > 4.0f) camera_.zoom = 4.0f;
  }
}
