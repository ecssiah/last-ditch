#ifndef CAMERA_SYSTEM_H
#define CAMERA_SYSTEM_H

#include "../components/Input.h"
#include "../components/Window.h"
#include "../components/Camera.h"

class CameraSystem
{
public:
  CameraSystem(Input& input, Window& window, Camera& camera);

  void Initialize();
  void Update();

private:
  Input& input_;
  Window& window_;
  Camera& camera_;
};

#endif // CAMERA_SYSTEM_H
