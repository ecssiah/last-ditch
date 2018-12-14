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
  map_.layers["floor"] = Layer();
  map_.layers["wall"] = Layer();
  map_.layers["obj"] = Layer();
  map_.layers["chr"] = Layer();
  map_.layers["overlay"] = Layer();

  for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
    for (auto y{0}; y < TILES_PER_LAYER; ++y) {
      SetTile("floor", x, y, "concrete");
    }
  }

  SetTile("wall", 8, 8, "wall2-str");
  SetTile("wall", 9, 8, "wall2-tee");
  SetTile("wall", 10, 8, "wall2-str");
  SetTile("wall", 11, 8, "door1");
  SetTile("wall", 12, 8, "wall2-cor");

  SetTile("ovr", 10, 10, "highlight");
  SetTile("ovr", 9, 8, "selection");

  SetTile("chr", 10, 10, "test_character1");
}

void MapSystem::SetTile(
  string layer, int x, int y, string type, float rotation, SDL_RendererFlip flip
) {
  Tile& tile = map_.layers[layer].tiles[x][y];

  if (TileData.find(type) != TileData.end()) {
    tile.active = true;
    tile.rotation = rotation;
    tile.flip = flip;

    tile.src.x = TileData[type].uv[0] * TILE_SIZE;  
    tile.src.y = TileData[type].uv[1] * TILE_SIZE;
  } else {
    cerr << "Tile(" << x << "," << y << ") has invalid type: " << type << endl; 
  }
}
