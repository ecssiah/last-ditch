#ifndef CAMERA_H
#define CAMERA_H

#include <algorithm>
#include <glm/glm.hpp>

#include "../utility/Types.h"
#include "../constants/CameraConstants.h"

class Camera
{
public:
  Camera()
    : speed{CAMERA_SPEED}
    , zoom{1.0f}
    , inv_zoom{1.0f / zoom}
    , pos{0, 0}
    , xdir{1, 0}
    , ydir{0, 1}
  {}

  f32 speed, zoom, inv_zoom;
  glm::vec2 pos, xdir, ydir;

};

#endif
