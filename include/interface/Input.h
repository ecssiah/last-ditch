#ifndef INPUT_H
#define INPUT_H

#include "../utility/Types.h"

struct Input
{
  Input() 
    : exit{false}
    , pause{false}
    , debug{false}
    , menu{false}
    , hud{true}
    , up{false}, down{false}, left{false}, right{false}
    , mag{false}, min{false}
    , ascend{false}, descend{false}
    , lclick{false}, mclick{false}, rclick{false}
    , lpressed{false}, mpressed{false}, rpressed{false}
    , lreleased{false}, mreleased{false}, rreleased{false}
    , mx{0}, my{0}
    , sx{0}, sy{0}
  {}

  bool exit, pause, debug, menu, hud;
  bool up, down, left, right; 
  bool mag, min;
  bool ascend, descend;
  bool lclick, mclick, rclick;
  bool lpressed, mpressed, rpressed;
  bool lreleased, mreleased, rreleased;

  i32 mx, my;
  i32 sx, sy;
};

#endif
