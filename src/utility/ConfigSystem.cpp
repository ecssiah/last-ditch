#include <iostream>
#include <boost/algorithm/string.hpp>

#include "../../include/utility/Logging.h"
#include "../../include/utility/ConfigSystem.h"
#include "../../include/map/MapConstants.h"
#include "../../include/map/Tile.h"

using namespace std;

ConfigSystem::ConfigSystem(Map& map)
  : map_{map}
{

}

void ConfigSystem::init()
{
  cout << "ConfigSystem initializing" << endl;

  load_tile_info();
}

void ConfigSystem::load_tile_info()
{
  const YAML::Node& tileset_map{YAML::LoadFile("assets/scripts/tiles.yml")};

  for (const auto& tileset_data : tileset_map) {
    const YAML::Node& category_map{tileset_data.second};   

    for (const auto& category_data : category_map) {
      const YAML::Node& tile_map{category_data.second};
      const auto category{category_data.first.as<string>()};

      for (const auto& tile_data : tile_map) {
        const YAML::Node& tile_node{tile_data.second};

        TileInfo tile_info;
        tile_info.category = category;

        const auto uv_vec{tile_node["uv"].as<vector<i32> >()};
        tile_info.uv.x = uv_vec[0];
        tile_info.uv.y = uv_vec[1];

        if (tile_info.category == "windows") {
          tile_info.border = tile_node["border"].as<i32>();
        }

        map_.tile_data[tile_data.first.as<string>()] = tile_info;
      }
    } 
  }
}
