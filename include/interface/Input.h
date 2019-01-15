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
    , mpx{0}, mpy{0}
    , mrx{0}, mry{0}
    , mdx{0}, mdy{0}
    , tx{0}, ty{0}
    , tdx{0}, tdy{0}
    , selectx{-1}, selecty{-1}, selectfloor{-1}
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
  i32 mpx, mpy;
  i32 mrx, mry;
  i32 mdx, mdy;

  f32 tx, ty;
  f32 tdx, tdy;

  i32 selectx, selecty, selectfloor;
};

#endif
