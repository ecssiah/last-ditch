#ifndef ROOM_H
#define ROOM_H

#include <string>

struct Room
{
  Room()
  {}

  Room(unsigned _l, unsigned _r, unsigned _t, unsigned _b)
    : l(_l)
    , r(_r)
    , t(_t)
    , b(_b)
  {}

  unsigned l, r, t, b;

  std::string wall_type;
  std::string floor_type;
};

#endif // ROOM_H
