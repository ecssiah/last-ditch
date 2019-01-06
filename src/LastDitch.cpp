#include <iostream>

#include "../include/LastDitch.h"

LastDitch::LastDitch()
  : config_system_{}
  , input_system_{input_, camera_, render_}
  , time_system_{input_, render_, time_}
  , render_system_{input_, render_, camera_, map_, ui_}
  , camera_system_{input_, render_, camera_}
  , map_system_{input_, map_}
  , entity_system_{map_}
  , ui_system_{input_, render_, map_, time_, log_, ui_} 
  , file_system_{input_, map_, time_}
{
  config_system_.init();
  time_system_.init();
  camera_system_.init();
  render_system_.init();
  ui_system_.init();
  input_system_.init();
  map_system_.init();
  entity_system_.init();
  file_system_.init();

  while (!input_.exit) {
    time_system_.frame_begin();

    input_system_.update();
    camera_system_.update();
    map_system_.update();
    entity_system_.update();
    ui_system_.update();
    render_system_.update();

    time_system_.frame_end();
  }
}


int main(int argc, char *argv[])
{
  LastDitch last_ditch;

  return 0;
}