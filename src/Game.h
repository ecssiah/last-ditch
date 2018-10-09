#ifndef GAME_H
#define GAME_H

#include "components/Input.h"
#include "systems/TimeSystem.h"
#include "systems/InputSystem.h"
#include "systems/RenderSystem.h"

class Game 
{
public:
  Game();
  ~Game();

  void Initialize();

private:
  Input input;

  TimeSystem time_system;
  InputSystem input_system;
  RenderSystem render_system;
};

#endif // GAME_H

