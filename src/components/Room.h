#ifndef ROOM_H
#define ROOM_H

#include <string>

struct Room
{
  Room(unsigned _x, unsigned _y, unsigned _w, unsigned _h)
    : x(_x)
    , y(_y)
    , w(_w)
    , h(_h)
  {}

  unsigned x, y, w, h;

  std::string wall_type;
  std::string floor_type;
};

#endif // ROOM_H
