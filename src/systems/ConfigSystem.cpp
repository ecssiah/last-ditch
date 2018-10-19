#include "ConfigSystem.h"

ConfigSystem::ConfigSystem()
{

}

void ConfigSystem::Initialize()
{

}

void ConfigSystem::LoadTileInfo()
{
  YAML::Node tile_data(YAML::LoadFile("scripts/tiles.yml"));
}
