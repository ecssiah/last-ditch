#ifndef LAST_DITCH_H
#define LAST_DITCH_H

#include "components/Render.h"
#include "components/Input.h"
#include "components/Camera.h"
#include "components/map/Map.h"
#include "systems/ConfigSystem.h"
#include "systems/RenderSystem.h"
#include "systems/MapSystem.h"
#include "systems/EntitySystem.h"
#include "systems/CameraSystem.h"
#include "systems/TimeSystem.h"
#include "systems/InputSystem.h"

class LastDitch 
{
public:
  LastDitch();

private:
  Render render_;
  Input input_;
  Camera camera_;
  Map map_;
  Time time_;

  ConfigSystem config_system_;
  RenderSystem render_system_;
  MapSystem map_system_;
  EntitySystem entity_system_;
  CameraSystem camera_system_;
  TimeSystem time_system_;
  InputSystem input_system_;
};

#endif // LAST_DITCH_H

