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
    YAML::Node tile_map(tileset.second);   

    for (auto tile_data : tile_map) {
      YAML::Node tile_node(tile_data.second);

      TileInfo tile_info;
      tile_info.uv = tile_node["uv"].as<vector<int> >();

      TileData[tile_data.first.as<string>()] = tile_info;
    } 
  }
}
