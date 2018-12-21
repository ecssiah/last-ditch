#ifndef TIME_H
#define TIME_H

struct Time
{
  Time()
    : time_changed{false}
    , date_changed{false}
    , ticks{0}
    , year{2612}
    , month{12}
    , day{30}
    , hour{23}
    , minute{59}
    , second{50}
  {}

  bool time_changed;
  bool date_changed;

  unsigned ticks; 
  unsigned year, month, day;
  unsigned hour, minute, second;

};

#endif
