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
    , touch_points{0}
    , mx{0}, my{0}
    , mdx{0}, mdy{0}
    , tx{0}, ty{0}
    , tdx{0}, tdy{0}
    , sx{0}, sy{0}
  {}

  bool exit, pause, debug, menu, hud;
  bool up, down, left, right; 
  bool mag, min;
  bool ascend, descend;
  bool lclick, mclick, rclick;
  bool lpressed, mpressed, rpressed;
  bool lreleased, mreleased, rreleased;

  i32 touch_points;

  i32 mx, my;
  i32 mdx, mdy;

  f32 tx, ty;
  f32 tdx, tdy;

  i32 sx, sy;
};

#endif
