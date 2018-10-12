#ifndef INPUT_H
#define INPUT_H

struct Input
{
  Input() : 
    up(false),
    down(false),
    left(false),
    right(false),
    exit(false),
    pause(false)
  {}

  bool up, down, left, right;
  bool exit;
  bool pause;
};

#endif // INPUT_H
