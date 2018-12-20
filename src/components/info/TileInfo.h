#ifndef TILE_INFO_H
#define TILE_INFO_H

#include <vector>
#include <glm/glm.hpp>

struct TileInfo
{
  TileInfo()
    : subtype{}
    , category{}
    , uv{2}
  {}

  std::string subtype;
  std::string category;

  std::vector<int> uv;
  glm::vec2 uv2;
};

#endif // TILE_INFO_H
