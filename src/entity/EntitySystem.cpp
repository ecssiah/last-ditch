#include "../../include/entity/EntitySystem.h"

#include "../../include/map/MapConstants.h"

using namespace std;

EntitySystem::EntitySystem(Map& map)
  : map_{map}
  , entities_{(u16)NUM_FLOORS}
{
}

void EntitySystem::init()
{
  cout << "EntitySystem init" << endl;
}

void EntitySystem::update()
{

}
