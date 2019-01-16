#ifndef TILE_INFO_H
#define TILE_INFO_H

#include <glm/glm.hpp>

struct TileInfo
{
  TileInfo()
    : subtype{}
    , category{}
    , uv{}
    , border{}
  {}

  std::string subtype;
  std::string category;

  glm::vec2 uv;
  
  i32 border;
};

#endif
