#include <iostream>

#include "MapSystem.h"

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
  std::cout << map_.chunks[0][4].tiles[3][2].type << std::endl;



}
