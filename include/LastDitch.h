#ifndef LAST_DITCH_H
#define LAST_DITCH_H

#include "render/Render.h"
#include "Input.h"
#include "render/Camera.h"
#include "map/Map.h"
#include "ConfigSystem.h"
#include "render/RenderSystem.h"
#include "render/CameraSystem.h"
#include "map/MapSystem.h"
#include "EntitySystem.h"
#include "ui/UISystem.h"
#include "TimeSystem.h"
#include "InputSystem.h"
#include "FileSystem.h"

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
  UISystem ui_system_;
  CameraSystem camera_system_;
  TimeSystem time_system_;
  InputSystem input_system_;
  FileSystem file_system_;
};

#endif
