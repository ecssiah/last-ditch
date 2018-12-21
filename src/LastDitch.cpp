#include <iostream>

#include "LastDitch.h"

LastDitch::LastDitch()
  : config_system_{}
  , input_system_{input_, render_}
  , time_system_{input_, render_, time_}
  , render_system_{input_, render_, camera_, map_}
  , camera_system_{input_, render_, camera_}
  , map_system_{input_, map_}
  , entity_system_{map_}
  , ui_system_{input_, render_, map_, time_} 
  , file_system_{input_, map_}
{
  config_system_.Initialize();
  time_system_.Initialize();
  camera_system_.Initialize();
  render_system_.Initialize();
  ui_system_.Initialize();
  input_system_.Initialize();
  map_system_.Initialize();
  entity_system_.Initialize();
  file_system_.Initialize();

  while (!input_.exit) {
    time_system_.StartFrame();

    camera_system_.Update();
    map_system_.Update();
    entity_system_.Update();
    render_system_.Update();
    ui_system_.Update();
    input_system_.Update();

    render_system_.Display();

    time_system_.EndFrame();
  }
}

