#ifndef CAMERA_H
#define CAMERA_H

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

struct Camera
{
  Camera() 
  {
  }

  float speed;
  glm::vec3 pos, x_dir, y_dir, z_dir;
};

#endif // CAMERA_H
