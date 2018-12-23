#ifndef TIME_H
#define TIME_H

#include <boost/serialization/access.hpp>

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

  int ticks; 
  int year, month, day;
  int hour, minute, second;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const unsigned int version)
  {
    ar & year;
    ar & month;
    ar & day;
    ar & hour;
    ar & minute;
    ar & second;
  }

};

#endif
