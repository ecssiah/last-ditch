#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "CameraSystem.h"

CameraSystem::CameraSystem(Input& input, Camera& camera) 
  : input_(input)
  , camera_(camera)
{
}

void CameraSystem::Initialize()
{
  camera_.speed = 0.05f;
  camera_.pos = glm::vec3(0.0f, 0.0f, 2.0f);
  camera_.z_dir = glm::vec3(0.0f, 0.0f, -1.0f); 
  camera_.y_dir = glm::vec3(0.0f, 1.0f, 0.0f);
  camera_.x_dir = glm::normalize(glm::cross(camera_.z_dir, camera_.y_dir));
}

void CameraSystem::Update()
{
  if (input_.up)
    camera_.pos += camera_.speed * camera_.z_dir; 
  if (input_.down)
    camera_.pos -= camera_.speed * camera_.z_dir;
  if (input_.right)
    camera_.pos += camera_.speed * camera_.x_dir;
  if (input_.left)
    camera_.pos -= camera_.speed * camera_.x_dir; 
}
