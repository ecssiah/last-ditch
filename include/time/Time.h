#ifndef TIME_H
#define TIME_H

#include <boost/serialization/access.hpp>

#include "../utility/Types.h"

struct Time
{
  Time()
    : time_changed{false}
    , date_changed{false}
    , ticks{0}
    , day{30}, month{12}, year{2612}
    , second{50}, minute{59}, hour{23}
  {}

  bool time_changed;
  bool date_changed;

  i32 ticks; 
  i32 year, month, day;
  i32 hour, minute, second;

private:
  friend class boost::serialization::access;

  template<class Archive>
  void serialize(Archive& ar, const u32 version)
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
