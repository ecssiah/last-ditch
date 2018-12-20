#ifndef TILE_INFO_H
#define TILE_INFO_H

#include <glm/glm.hpp>

struct TileInfo
{
  TileInfo()
    : subtype{}
    , category{}
    , uv{}
  {}

  std::string subtype;
  std::string category;

  glm::vec2 uv;
};

#endif // TILE_INFO_H
