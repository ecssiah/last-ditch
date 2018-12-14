#include <iostream>

#include "MapSystem.h"
#include "../constants/MapConstants.h"

using namespace std;

MapSystem::MapSystem(Map& map)
  : map_(map)
{
}

void MapSystem::Initialize()
{
  GenerateMap();
}

void MapSystem::Update()
{

}

void MapSystem::GenerateMap()
{
  map_.layers["map"] = Layer();
  map_.layers["obj"] = Layer();
  map_.layers["chr"] = Layer();

  for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
    for (auto y{0}; y < TILES_PER_LAYER; ++y) {
      // Generate Test Data
      if (x % 2 == 0) {
        SetTile("map", x, y, "floor1");
      } else {
        SetTile("map", x, y, "floor2");
        SetTile("obj", x, y, "test1");
      }

      if (y % 3 == 0) {
        SetTile("chr", x, y, "test_character1");
      }
    }
  }
}

void MapSystem::SetTile(string layer, int x, int y, string type) {
  Tile& tile = map_.layers[layer].tiles[x][y];
  tile.active = true;

  tile.src.x = TileData[type].uv[0] * TILE_SIZE;  
  tile.src.y = TileData[type].uv[1] * TILE_SIZE;
}
