#include "../../include/entity/EntitySystem.h"

#include "../../include/utility/Logging.h"
#include "../../include/constants/MapConstants.h"

using namespace std;

EntitySystem::EntitySystem(Map& map)
  : map_{map}
  , entities_{(u16)NUM_FLOORS}
{
}

void EntitySystem::init()
{
  cout << "EntitySystem initializing" << endl;
}

void EntitySystem::update()
{

}
