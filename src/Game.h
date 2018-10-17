#ifndef GAME_H
#define GAME_H

#include "components/Window.h"
#include "components/Input.h"
#include "components/Camera.h"
#include "components/Map.h"
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
  Window window_;
  Input input_;
  Camera camera_;
  Map map_;

  RenderSystem render_system_;
  MapSystem map_system_;
  CameraSystem camera_system_;
  TimeSystem time_system_;
  InputSystem input_system_;
};

#endif // GAME_H

