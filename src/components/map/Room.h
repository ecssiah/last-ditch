#ifndef ROOM_H
#define ROOM_H

#include <string>

struct Room
{
  Room()
  {}

  Room(unsigned _l, unsigned _r, unsigned _t, unsigned _b)
    : l{_l}
    , r{_r}
    , t{_t}
    , b{_b}
    , wall_type{}
    , floor_type{}
  {}

  bool operator ==(const Room& room) const {
    return l == room.l && r == room.r && t == room.t && b == room.b; 
  }

  bool operator !=(const Room& room) const {
    return !(*this == room); 
  }

  unsigned l, r, t, b;

  std::string wall_type;
  std::string floor_type;
};


#endif // ROOM_H
