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
  map_.layers["object"] = Layer();
  map_.layers["character"] = Layer();

  for (auto x{0}; x < TILES_PER_LAYER; ++x) { 
    for (auto y{0}; y < TILES_PER_LAYER; ++y) {
      if (x % 2 == 0) {
        map_.layers["map"].tiles[x][y].type = 1;
      } else {
        map_.layers["object"].tiles[x][y].type = 1;
      }

      if (y % 2 == 0) {
        map_.layers["character"].tiles[x][y].type = 1;
      }

      cout << map_.layers["map"].tiles[x][y].type;
      cout << map_.layers["object"].tiles[x][y].type;
      cout << map_.layers["character"].tiles[x][y].type;
      cout << " ";
    }
    cout << endl;
  }
}
