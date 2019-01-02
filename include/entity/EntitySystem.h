#ifndef ENTITY_SYSTEM_H
#define ENTITY_SYSTEM_H

#include <vector>

#include "Entity.h"
#include "../utility/Types.h"
#include "../map/Map.h"

class EntitySystem
{
  Map& map_;

  std::vector<Entity> entities_;

public:
  EntitySystem(Map& map);

  void init();
  void update();

};

#endif