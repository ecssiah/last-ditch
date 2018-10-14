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
    , exit(false)
    , pause(false)
    , debug(false)
  {}

  bool up, down, left, right, mag, min;
  bool exit, pause, debug;
};

#endif // INPUT_H
