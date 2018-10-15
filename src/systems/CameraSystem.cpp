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
  camera_.pos = glm::vec3(0.0f, 0.0f, 24.0f);
  camera_.z_dir = glm::vec3(0.0f, 0.0f, -1.0f); 
  camera_.y_dir = glm::vec3(0.0f, 1.0f, 0.0f);
  camera_.x_dir = glm::normalize(glm::cross(camera_.z_dir, camera_.y_dir));
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
    camera_.pos -= window_.dt * camera_.speed * camera_.z_dir;
  if (input_.mag)
    camera_.pos += window_.dt * camera_.speed * camera_.z_dir;
}
