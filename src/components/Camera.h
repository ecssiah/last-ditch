#ifndef CAMERA_H
#define CAMERA_H

#include <glm/glm.hpp>

#include "../Types.h"
#include "../constants/CameraConstants.h"

struct Camera
{
  Camera() 
    : speed{CAMERA_SPEED}
    , zoom{1.0f}
    , pos{0, 0}
    , xdir{1, 0}
    , ydir{0, 1}
  {
  }

  F32 speed, zoom;
  glm::vec2 pos, xdir, ydir;

};

#endif
