#ifndef INPUT_H
#define INPUT_H

struct Input
{
  Input() 
    : up(false)
    , down(false)
    , left(false)
    , right(false)
    , mag(false)
    , min(false)
    , lclick(false)
    , rclick(false)
    , exit(false)
    , pause(false)
    , debug(false)
    , mx(0)
    , my(0)
  {}

  bool up, down, left, right; 
  bool mag, min;
  bool lclick, rclick;
  bool exit, pause, debug;

  double mx, my;
};

#endif // INPUT_H
