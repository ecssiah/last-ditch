#include "MapGenerator.h"

#include "../components/Room.h"
#include "../constants/MapConstants.h"

using namespace std;

MapGenerator::MapGenerator(Map& map)
  : map_(map)
{
}

void MapGenerator::GenerateMap(string name)
{
  for (auto floor{0}; floor < NUM_FLOORS; ++floor) {
    for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
      for (auto y{0}; y < TILES_PER_LAYER; ++y) {
        SetTile("floor", x, y, floor, "concrete");
      }
    }

        
    
  }
}

void MapGenerator::SetTile(
  string layer, 
  int x, int y, int floor, 
  string type, 
  float rotation, SDL_RendererFlip flip
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
