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

  for (auto cx{0}; cx < CHUNKS_PER_LAYER; ++cx) { 
    for (auto cy{0}; cy < CHUNKS_PER_LAYER; ++cy) {
      for (auto x{0}; x < TILES_PER_CHUNK; ++x) {
        for (auto y{0}; y < TILES_PER_CHUNK; ++y) {
          if (x % 2 == 0) {
            map_.layers["map"].chunks[cx][cy].tiles[x][y].type = 1;
          } else {
            map_.layers["object"].chunks[cx][cy].tiles[x][y].type = 1;
          }

          if (y % 2 == 0) {
            map_.layers["character"].chunks[cx][cy].tiles[x][y].type = 1;
          }

          cout << map_.layers["map"].chunks[cx][cy].tiles[x][y].type;
          cout << map_.layers["object"].chunks[cx][cy].tiles[x][y].type;
          cout << map_.layers["character"].chunks[cx][cy].tiles[x][y].type;
          cout << " ";
        }
        cout << endl;
      }   
      cout << endl;
    }
  }
}
