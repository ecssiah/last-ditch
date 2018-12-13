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
        SetTile("map", x, y, TileData["floor1"].type);
      } else {
        SetTile("map", x, y, 2);
        SetTile("obj", x, y, 1);
      }

      if (y % 3 == 0) {
        SetTile("chr", x, y, 1);
      }
    }
  }
}

void MapSystem::SetTile(std::string layer, int x, int y, int type) {
  Tile& tile = map_.layers[layer].tiles[x][y];

  tile.type = type;

  tile.src.x = (tile.type - 1) % TILESET_WIDTH * TILE_SIZE;  
  tile.src.y = (tile.type - 1) / TILESET_HEIGHT * TILE_SIZE;
}
