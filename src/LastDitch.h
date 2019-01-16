#ifndef LAST_DITCH_H
#define LAST_DITCH_H

#include "interface/Input.h"
#include "interface/InputSystem.h"
#include "time/TimeSystem.h"
#include "utility/Log.h"
#include "utility/FileSystem.h"
#include "utility/ConfigSystem.h"
#include "render/Render.h"
#include "render/Camera.h"
#include "render/RenderSystem.h"
#include "render/CameraSystem.h"
#include "map/Map.h"
#include "map/MapSystem.h"
#include "ui/UI.h"
#include "ui/UISystem.h"
#include "entity/EntitySystem.h"

class LastDitch 
{
public:
  LastDitch();

private:
  ConfigSystem config_system_;
  RenderSystem render_system_;
  MapSystem map_system_;
  EntitySystem entity_system_;
  UISystem ui_system_;
  CameraSystem camera_system_;
  TimeSystem time_system_;
  InputSystem input_system_;
  FileSystem file_system_;

  Render render_;
  Camera camera_;
  Input input_;
  Time time_;
  Map map_;
  Log log_;
  UI ui_;

};

#endif

