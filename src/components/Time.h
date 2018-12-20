#ifndef TIME_H
#define TIME_H

struct Time
{
  Time()
    : has_changed{false}
    , ticks{0}
    , year{2612}
    , month{1}
    , day{12}
    , hour{12}
    , minute{12}
    , second{12}
  {}

  bool has_changed;

  unsigned ticks; 
  unsigned year, month, day;
  unsigned hour, minute, second;

};

#endif // TIME_H
