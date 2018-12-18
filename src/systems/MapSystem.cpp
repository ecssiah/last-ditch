#include <iostream>
#include <fstream>
#include <boost/archive/text_oarchive.hpp>
#include <boost/archive/text_iarchive.hpp>

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
  /* map_generator_.GenerateMap("test_map"); */

  /* SaveMap("test_map1"); */
  LoadMap("test_map1");
}


void MapSystem::Update()
{
  if (input_.ascend && map_.cur_floor < NUM_FLOORS - 1) map_.cur_floor++; 
  if (input_.descend && map_.cur_floor > 0) map_.cur_floor--;
}


void MapSystem::SaveMap(std::string filename)
{
  ofstream ofs("assets/maps/" + filename);

  boost::archive::text_oarchive oa(ofs);
  oa << map_;

  cout << "Map saved as: " << filename << endl;
}


bool MapSystem::LoadMap(std::string filename)
{
  ifstream ifs("assets/maps/" + filename);

  if (ifs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::text_iarchive ia(ifs);
    ia >> map_;

    cout << "Map loaded: " << filename << endl;

    return true;
  }
}

