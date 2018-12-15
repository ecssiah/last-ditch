#include <iostream>

#include "MapSystem.h"
#include "../constants/MapConstants.h"

using namespace std;

MapSystem::MapSystem(Input& input, Map& map)
  : input_(input)
  , map_(map)
  , map_generator_(map)
{
}

void MapSystem::Initialize()
{
  map_.floors.resize(NUM_FLOORS);

  for (auto i{0}; i < NUM_FLOORS; ++i) {
    map_.floors[i].layers["floor"] = Layer();
    map_.floors[i].layers["wall"] = Layer();
    map_.floors[i].layers["object"] = Layer();
    map_.floors[i].layers["entity"] = Layer();
    map_.floors[i].layers["overlay"] = Layer();
  }

  GenerateMap();
}

void MapSystem::Update()
{
  if (input_.ascend && map_.cur_floor < NUM_FLOORS - 1) map_.cur_floor++; 
  if (input_.descend && map_.cur_floor > 0) map_.cur_floor--;
}

void MapSystem::GenerateMap()
{
  for (auto floor{0}; floor < NUM_FLOORS; ++floor) {
    for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
      for (auto y{0}; y < TILES_PER_LAYER; ++y) {
        SetTile("floor", x, y, floor, "concrete");
      }
    }
  }

  SetTile("wall", 7, 8, 0, "wall1-end", 0, SDL_FLIP_HORIZONTAL);
  SetTile("wall", 8, 8, 0, "wall1-str");
  SetTile("wall", 9, 8, 0, "wall1-tee");
  SetTile("wall", 9, 9, 0, "wall1-str", 90);
  SetTile("wall", 9, 10, 0, "wall1-end", 90);
  SetTile("wall", 10, 8, 0, "wall1-str");
  SetTile("wall", 11, 8, 0, "door1");
  SetTile("wall", 12, 8, 0, "wall1-cor");
  SetTile("wall", 12, 9, 0, "wall1-end", 90);
  SetTile("overlay", 9, 8, 0, "selection");
  SetTile("entity", 10, 10, 0, "test_character1");

  SetTile("wall", 10, 10, 1, "door1");

  SetTile("wall", 10, 12, 2, "wall1-str");

  SetTile("wall", 8, 10, 3, "wall2-str");
}

void MapSystem::SetTile(
  string layer, 
  int x, int y, int floor, 
  string type, float rotation, SDL_RendererFlip flip
) {
  Tile& tile = map_.floors[floor].layers[layer].tiles[x][y];

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
