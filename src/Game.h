#ifndef GAME_H
#define GAME_H

#include "components/Input.h"
#include "components/Window.h"
#include "components/Camera.h"
#include "systems/RenderSystem.h"
#include "systems/MapSystem.h"
#include "systems/CameraSystem.h"
#include "systems/TimeSystem.h"
#include "systems/InputSystem.h"

class Game 
{
public:
  Game();
  ~Game();

  void Initialize();

private:
  Input input_;
  Camera camera_;
  Window window_;

  RenderSystem render_system_;
  MapSystem map_system_;
  CameraSystem camera_system_;
  TimeSystem time_system_;
  InputSystem input_system_;
};

#endif // GAME_H

