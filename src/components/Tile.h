#ifndef TILE_H
#define TILE_H

struct Tile
{
  Tile()
    : type(0)
    , category(0)
  {}

  unsigned category;
  unsigned type;
};

#endif // TILE_H
