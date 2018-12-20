#include <iostream>
#include <boost/algorithm/string.hpp>

#include "ConfigSystem.h"
#include "../constants/MapConstants.h"
#include "../components/map/Tile.h"

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
  YAML::Node tileset_map{YAML::LoadFile("assets/scripts/tiles.yml")};

  for (auto tileset_data : tileset_map) {
    YAML::Node category_map{tileset_data.second};   

    for (auto category_data : category_map) {
      YAML::Node tile_map{category_data.second};

      auto category{category_data.first.as<string>()};

      for (auto tile_data : tile_map) {
        YAML::Node tile_node{tile_data.second};

        TileInfo tile_info;
        tile_info.category = category;
        tile_info.uv = tile_node["uv"].as<vector<int> >();

        TileData[tile_data.first.as<string>()] = tile_info;
      }
    } 
  }
}
