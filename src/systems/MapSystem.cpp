#include <iostream>
#include <fstream>
#include <boost/archive/binary_oarchive.hpp>
#include <boost/archive/binary_iarchive.hpp>

#include "MapSystem.h"
#include "../constants/MapConstants.h"

using namespace std;

MapSystem::MapSystem(Input& input, Map& map)
  : input_{input}
  , map_{map}
  , map_generator_{map}
{
}


void MapSystem::Initialize()
{
  map_generator_.GenerateMap("test_map");

  SaveMap("test_map1");
  /* LoadMap("test_map1"); */
}


void MapSystem::Update()
{
  if (map_.floor_changed) map_.floor_changed = false;

  if (input_.descend && map_.cur_floor > 0) {
    map_.cur_floor--;
    map_.floor_changed = true;
  }
  if (input_.ascend && map_.cur_floor < NUM_FLOORS - 1) {
    map_.cur_floor++; 
    map_.floor_changed = true;
  }
}


bool MapSystem::SaveMap(std::string filename)
{
  ofstream ofs("assets/maps/" + filename);

  if (ofs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_oarchive oa(ofs);
    oa << map_;

    cout << "Map saved as: " << filename << endl;

    return true;
  }
}


bool MapSystem::LoadMap(std::string filename)
{
  ifstream ifs("assets/maps/" + filename);

  if (ifs.fail()) {
    cerr << "Error: " << strerror(errno);

    return false;
  } else {
    boost::archive::binary_iarchive ia(ifs);
    ia >> map_;

    cout << "Map loaded: " << filename << endl;

    return true;
  }
}

