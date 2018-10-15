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
  camera_.speed = 4.5f;
  camera_.zoom = 1.0f;
  camera_.pos = {0.0f, 0.0f, 1.0f};
  camera_.x_dir = {-1.0f, 0.0f, 0.0f};
  camera_.y_dir = {0.0f, 1.0f, 0.0f};
  camera_.z_dir = {0.0f, 0.0f, -1.0f}; 
}

void CameraSystem::Update()
{
  if (input_.up)
    camera_.pos += window_.dt * camera_.speed * camera_.y_dir; 
  if (input_.down)
    camera_.pos -= window_.dt * camera_.speed * camera_.y_dir;
  if (input_.left)
    camera_.pos -= window_.dt * camera_.speed * camera_.x_dir; 
  if (input_.right)
    camera_.pos += window_.dt * camera_.speed * camera_.x_dir;
  if (input_.min)
    camera_.zoom -= window_.dt;
  if (input_.mag)
    camera_.zoom += window_.dt;
}
