#ifndef CAMERA_H
#define CAMERA_H

#include <algorithm>
#include <glm/glm.hpp>

#include "../Types.h"
#include "../constants/MapConstants.h"
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
  {
  }

  inline void move(f32 dt, Dirs dir)
  {
    auto modifier{inv_zoom * speed * dt};

    switch (dir) {
    case RIGHT: pos += modifier * xdir; break;
    case UP:    pos -= modifier * ydir; break;
    case LEFT:  pos -= modifier * xdir; break;
    case DOWN:  pos += modifier * ydir; break;
    };
  }

  inline void inc_zoom() 
  { 
    zoom = std::min(MAX_ZOOM, zoom * 2); 
    inv_zoom = 1.0 / zoom;
  }

  inline void dec_zoom() 
  { 
    zoom = std::max(MIN_ZOOM, zoom / 2); 
    inv_zoom = 1.0 / zoom;
  }

  f32 speed, zoom, inv_zoom;
  glm::vec2 pos, xdir, ydir;

};

#endif
