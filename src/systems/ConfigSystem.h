#ifndef CONFIG_SYSTEM_H
#define CONFIG_SYSTEM_H

#include <yaml-cpp/yaml.h>

#include "../Types.h"

class ConfigSystem
{
public:
  ConfigSystem();

  void init();

private:
  void load_tile_info();

}; 

#endif
