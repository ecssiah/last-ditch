#ifndef CAMERA_H
#define CAMERA_H

#include <algorithm>
#include <glm/glm.hpp>

#include "../Types.h"
#include "../map/MapConstants.h"
#include "../render/CameraConstants.h"

class Camera
{
public:
  Camera();

  void move(f32 dt, Dirs dir);
  void inc_zoom();
  void dec_zoom();

  f32 speed, zoom, inv_zoom;
  glm::vec2 pos, xdir, ydir;

};

#endif
