#ifndef ENTITY_SYSTEM_H
#define ENTITY_SYSTEM_H

#include <vector>

#include "Entity.h"
#include "../map/Map.h"
#include "../utility/Types.h"

class EntitySystem
{
public:
  EntitySystem(Map& map);

  void init();
  void update();

private:
  Map& map_;

  std::vector<Entity> entities_;

};

#endif
