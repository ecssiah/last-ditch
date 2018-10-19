#ifndef CONFIG_SYSTEM_H
#define CONFIG_SYSTEM_H

#include <yaml-cpp/yaml.h>

class ConfigSystem
{
public:
  ConfigSystem();

  void Initialize();

private:
  void LoadTileInfo();

}; 

#endif // CONFIG_SYSTEM_H
