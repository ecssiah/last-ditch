#include <algorithm>

#include "TimeSystem.h"
#include "../constants/SimulationConstants.h"

using namespace std;

TimeSystem::TimeSystem(Input& input, Render& render, Time& time)
  : input_{input}
  , render_{render}
  , time_{time}
{
}


void TimeSystem::Initialize()
{
}


void TimeSystem::StartFrame()
{
  start_ = chrono::steady_clock::now();
}


void TimeSystem::EndFrame()
{
  end_ = chrono::steady_clock::now();

  auto microseconds{ 
    chrono::duration_cast<chrono::microseconds>(end_ - start_).count()
  };

  render_.dt = min(MAX_DELTA_TIME, 1e-6 * microseconds);

  if (!input_.pause) {
    Tick();
  }
}


void TimeSystem::Tick()
{
  time_.ticks += 1;
  time_.time_changed = false;
  time_.date_changed = false;

  if (time_.ticks > TICKS_PER_SECOND) {
    time_.time_changed = true;

    time_.ticks = 0;
    time_.second += 1;

    if (time_.second >= SECONDS_PER_MINUTE) {
      time_.second = 0;
      time_.minute += 1;

      if (time_.minute >= MINUTES_PER_HOUR) {
        time_.minute = 0;
        time_.hour += 1;

        if (time_.hour >= HOURS_PER_DAY) {
          time_.date_changed = true;
          time_.hour = 0;
          time_.day += 1;

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

