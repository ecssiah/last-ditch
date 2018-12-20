#ifndef INPUT_H
#define INPUT_H

struct Input
{
  Input() 
    : up{false}
    , down{false}
    , left{false}
    , right{false}
    , mag{false}
    , min{false}
    , ascend{false}
    , descend{false}
    , lclick{false}
    , mclick{false}
    , rclick{false}
    , lpressed{false}
    , mpressed{false}
    , rpressed{false}
    , exit{false}
    , pause{false}
    , debug{false}
    , menu{false}
    , mx{0}
    , my{0}
  {}

  bool up, down, left, right; 
  bool mag, min;
  bool ascend, descend;
  bool lclick, mclick, rclick;
  bool lpressed, mpressed, rpressed;
  bool exit, pause, debug, menu;

  int mx, my;
};

#endif // INPUT_H
