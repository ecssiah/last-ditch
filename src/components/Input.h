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
    , mx(0.0f)
    , my(0.0f)
    , exit(false)
    , pause(false)
    , debug(false)
  {}

  bool up, down, left, right, mag, min;
  bool lclick, rclick;
  bool exit, pause, debug;

  double mx, my;
};

#endif // INPUT_H
