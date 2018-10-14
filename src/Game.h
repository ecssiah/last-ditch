#ifndef GAME_H
#define GAME_H

#include "components/Input.h"
#include "components/Window.h"
#include "systems/RenderSystem.h"
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
  Window window_;

  RenderSystem render_system;
  CameraSystem camera_system;
  TimeSystem time_system;
  InputSystem input_system;
};

#endif // GAME_H

