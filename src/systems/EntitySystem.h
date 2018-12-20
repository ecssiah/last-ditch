#ifndef ENTITY_SYSTEM_H
#define ENTITY_SYSTEM_H

#include <vector>

#include "../components/Entity.h"
#include "../components/map/Map.h"

class EntitySystem
{
public:
  EntitySystem(Map& map);

  void Initialize();
  void Update();

private:
  Map& map_;

  std::vector<Entity> entities_;
};

#endif // ENTITY_SYSTEM_H
