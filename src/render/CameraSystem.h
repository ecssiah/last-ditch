#ifndef CAMERA_SYSTEM_H
#define CAMERA_SYSTEM_H

#include "../interface/Input.h"
#include "../render/Render.h"
#include "../render/Camera.h"

class CameraSystem
{
public:
  CameraSystem(Input& input, Render& render, Camera& camera);

  void init();
  void update();

private:
  void move(Dir dir);

  void inc_zoom();
  void dec_zoom();

  void update_scale();

  Input& input_;
  Render& render_;
  Camera& camera_;
};

#endif
