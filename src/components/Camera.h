#ifndef CAMERA_H
#define CAMERA_H

#include <glm/glm.hpp>

struct Camera
{
  Camera() 
  {
  }

  float speed, zoom;
  glm::vec2 pos, xdir, ydir;
};

#endif // CAMERA_H
