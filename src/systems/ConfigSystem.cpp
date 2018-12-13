#include <iostream>

#include "ConfigSystem.h"
#include "../components/Tile.h"
#include "../constants/MapConstants.h"

using namespace std;

ConfigSystem::ConfigSystem()
{

}

void ConfigSystem::Initialize()
{
  LoadTileInfo();
}

void ConfigSystem::LoadTileInfo()
{
  YAML::Node tileset_data = YAML::LoadFile("assets/scripts/tiles.yml");

  for (auto tileset : tileset_data) {
    auto type(tileset.first.as<string>());

    YAML::Node tile_array(tileset.second);   

    for (auto i{0}; i < tile_array.size(); ++i) {
      TileInfo tile_info;
      tile_info.type = i + 1;

      TileData[tile_array[i].as<string>()] = tile_info;
    } 
  }
}
