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

  map_generator_.GenerateMap("test_map");
}


void MapSystem::Update()
{
  if (input_.ascend && map_.cur_floor < NUM_FLOORS - 1) map_.cur_floor++; 
  if (input_.descend && map_.cur_floor > 0) map_.cur_floor--;
}


void MapSystem::SaveMap(std::string filename)
{

}


bool MapSystem::LoadMap(std::string filename)
{

}

