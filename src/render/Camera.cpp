#include "../../include/render/Camera.h"

Camera::Camera()
  : speed{CAMERA_SPEED}
  , zoom{1.0f}
  , inv_zoom{1.0f / zoom}
  , pos{0, 0}
  , xdir{1, 0}
  , ydir{0, 1}
{
}


void Camera::move(f32 dt, Dirs dir)
{
  auto ds{speed * dt * inv_zoom};

  switch (dir) {
  case UP:    pos -= ds * ydir; break;
  case DOWN:  pos += ds * ydir; break;
  case LEFT:  pos -= ds * xdir; break;
  case RIGHT: pos += ds * xdir; break;
  };
}


void Camera::inc_zoom() 
{ 
  zoom = std::min(MAX_ZOOM, zoom * 2); 
  inv_zoom = 1.0 / zoom;
}


void Camera::dec_zoom() 
{ 
  zoom = std::max(MIN_ZOOM, zoom / 2); 
  inv_zoom = 1.0 / zoom;
}
