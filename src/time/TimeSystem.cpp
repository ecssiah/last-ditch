#include <algorithm>

#include "../../include/constants/TimeConstants.h"
#include "../../include/time/TimeSystem.h"

using namespace std;

TimeSystem::TimeSystem(Input& input, Render& render, Time& time)
  : input_{input}
  , render_{render}
  , time_{time}
  , ticks_{0}
{
}


void TimeSystem::frame_begin()
{
  start_ = chrono::steady_clock::now();
}


void TimeSystem::frame_end()
{
  if (!input_.pause) tick();

  end_ = chrono::steady_clock::now();
  auto elapsed{chrono::duration_cast<chrono::duration<float>>(end_ - start_)};

  render_.dt = elapsed.count();
}


void TimeSystem::tick()
{
  ticks_ += 1;
  time_.time_changed = false;
  time_.date_changed = false;

  if (ticks_ >= TICKS_PER_SECOND) {
    ticks_ = 0;
    time_.second += 1;
    time_.time_changed = true;

    if (time_.second >= SECONDS_PER_MINUTE) {
      time_.second = 0;
      time_.minute += 1;

      if (time_.minute >= MINUTES_PER_HOUR) {
        time_.minute = 0;
        time_.hour += 1;

        if (time_.hour >= HOURS_PER_DAY) {
          time_.hour = 0;
          time_.day += 1;
          time_.date_changed = true;

          if (time_.day > DAYS_PER_MONTH) {
            time_.day = 1;
            time_.month += 1;

            if (time_.month > MONTHS_PER_YEAR) {
              time_.month = 1;
              time_.year += 1;
            }
          }
        }
      }
    }
  }
}

