#ifndef TILE_INFO_H
#define TILE_INFO_H

#include <vector>

struct TileInfo
{
  TileInfo()
    : uv()
  {}

  std::vector<int> uv;
};

#endif // TILE_INFO_H