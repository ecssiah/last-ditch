#ifndef CONFIG_SYSTEM_H
#define CONFIG_SYSTEM_H

#include <yaml-cpp/yaml.h>

#include "Types.h"
#include "../../include/map/Map.h"

class ConfigSystem
{
public:
  ConfigSystem(Map& map);

  void init();

private:
  void load_tile_info();

  Map& map_;

}; 

#endif
